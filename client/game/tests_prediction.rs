#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::client::game::prediction::Prediction;
    use crate::libraries::events::EventManager;
    use crate::libraries::fixed::Fixed;
    use crate::shared::blocks::{Block, Blocks};
    use crate::shared::entities::{step_entity, PhysicsComponent, PositionComponent};
    use crate::shared::liquids::Liquids;
    use crate::shared::players::{step_player, MovingType, PlayerComponent, PlayerInput};

    fn world() -> (Blocks, Liquids) {
        let mut blocks = Blocks::new();
        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);
        blocks.create((40, 20));

        let mut events = EventManager::new();
        for x in 0..40 {
            for y in 15..20 {
                blocks.set_block(&mut events, x, y, solid_id).unwrap();
            }
        }

        let mut liquids = Liquids::new();
        liquids.create((40, 20));
        (blocks, liquids)
    }

    fn moving(moving_type: MovingType) -> PlayerInput {
        PlayerInput { moving_type, jumping: false }
    }

    /// Simulates a player forward, recording each tick the way the client's loop does.
    fn run(prediction: &mut Prediction, ticks: u64, input: PlayerInput, blocks: &Blocks, liquids: &Liquids) -> (PositionComponent, PhysicsComponent, PlayerComponent) {
        let mut position = PositionComponent::new(Fixed::from_int(5), Fixed::from_int(12));
        let mut physics = PhysicsComponent::new(Fixed::ONE, Fixed::from_int(2));
        let mut player = PlayerComponent::new("test");

        for tick in 1..=ticks {
            player.apply_input(input, &mut physics);
            step_player(&position, &mut physics, &mut player, blocks, liquids);
            step_entity(&mut position, &mut physics, blocks, liquids);
            prediction.record(tick, input, position, physics);
        }
        (position, physics, player)
    }

    /// The common case, and the one that has to cost nothing: the server confirms a state
    /// this client already had, so there is no correction and nothing moves.
    #[test]
    fn test_a_matching_server_state_is_not_a_correction() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (_position, _physics, mut player) = run(&mut prediction, 40, moving(MovingType::MovingRight), &blocks, &liquids);

        let (agreed_position, agreed_physics) = prediction.state_at(20).unwrap();
        let correction = prediction.reconcile(20, agreed_position, agreed_physics, &mut player, &blocks, &liquids);

        assert!(correction.is_none(), "an agreeing server state should not correct anything");
        assert_eq!(prediction.corrections, 0);
    }

    /// The point of the whole mechanism. The server disagrees about an old tick; the client
    /// takes its answer for *that* tick and replays everything it has done since, so the
    /// correction does not throw away the last fifth of a second of the player's input.
    #[test]
    fn test_a_correction_replays_the_inputs_since_that_tick() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (predicted_position, _physics, mut player) = run(&mut prediction, 40, moving(MovingType::MovingRight), &blocks, &liquids);

        // the server puts the player a little to the left of where this client had it at 20
        let (mut server_position, server_physics) = prediction.state_at(20).unwrap();
        server_position.set_x(server_position.x() - Fixed::ONE);

        let (corrected, _corrected_physics) = prediction.reconcile(20, server_position, server_physics, &mut player, &blocks, &liquids).unwrap();

        assert_eq!(prediction.corrections, 1);
        // the correction moved the player back by about the block the server disagreed by...
        assert!(corrected.x() < predicted_position.x(), "the correction should have moved the player left");
        // ...and no further: the 20 ticks of movement since were replayed, not discarded
        let lost = predicted_position.x() - corrected.x();
        assert!(lost < Fixed::from_num(3, 2), "the replay lost {lost} blocks of movement, not just the one the server disagreed by");
    }

    /// Replaying reaches the same answer as never having been corrected, when the server
    /// agrees. This is `step_entity`'s determinism carried through the whole player step.
    #[test]
    fn test_replay_reproduces_the_original_when_the_server_agrees() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (original, _physics, mut player) = run(&mut prediction, 60, moving(MovingType::MovingRight), &blocks, &liquids);

        // Force a replay from tick 20 with a state that differs only in velocity, then put
        // the velocity back: the replayed path has to land exactly where the first one did.
        // The nudge has to be big enough to survive the divide by the tick rate - a change
        // below the resolution truncates to nothing, which is what makes velocities settle.
        let (position_20, physics_20) = prediction.state_at(20).unwrap();
        let mut nudged = physics_20;
        nudged.velocity_x += Fixed::ONE;
        prediction.reconcile(20, position_20, nudged, &mut player, &blocks, &liquids).unwrap();
        let after_nudge = prediction.state_at(60).unwrap().0;

        assert_ne!(after_nudge, original, "a different velocity should have produced a different path");

        let (position_again, _) = prediction.state_at(20).unwrap();
        prediction.reconcile(20, position_again, physics_20, &mut player, &blocks, &liquids);

        assert_eq!(prediction.state_at(60).unwrap().0, original, "replaying the original state must reproduce the original path");
    }

    /// A tick older than the buffer cannot be replayed onto, so it is ignored rather than
    /// snapped to - accepting it would undo every input since, which is the rubber-band.
    #[test]
    fn test_a_state_for_a_forgotten_tick_is_ignored() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (_position, _physics, mut player) = run(&mut prediction, 40, moving(MovingType::MovingRight), &blocks, &liquids);

        let stale = PositionComponent::new(Fixed::ZERO, Fixed::ZERO);
        let stale_physics = PhysicsComponent::new(Fixed::ONE, Fixed::from_int(2));

        assert!(prediction.reconcile(0, stale, stale_physics, &mut player, &blocks, &liquids).is_none());
        assert_eq!(prediction.corrections, 0);
    }

    /// The history is bounded, so a long session cannot grow it without limit.
    #[test]
    fn test_history_is_bounded() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        run(&mut prediction, 2000, moving(MovingType::Standing), &blocks, &liquids);

        assert!(prediction.history_len() <= 400, "history grew to {}", prediction.history_len());
    }

    /// A correction is taken by the simulation at once but paid off gradually on screen, so
    /// what the player sees is a slide rather than a jump.
    #[test]
    fn test_a_correction_is_smoothed_out_of_the_drawn_position() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (_position, _physics, mut player) = run(&mut prediction, 40, moving(MovingType::MovingRight), &blocks, &liquids);

        let (mut server_position, server_physics) = prediction.state_at(20).unwrap();
        server_position.set_x(server_position.x() - Fixed::ONE);
        prediction.reconcile(20, server_position, server_physics, &mut player, &blocks, &liquids).unwrap();

        assert_ne!(prediction.visual_offset().0, Fixed::ZERO, "the correction should be visible as an offset to work off");

        for _ in 0..500 {
            prediction.decay_visual_error();
        }

        assert_eq!(prediction.visual_offset(), (Fixed::ZERO, Fixed::ZERO), "the offset must reach zero, not approach it");
    }

    /// A forced state is the server deciding where the player is - a respawn, a teleport -
    /// not reporting where the simulation put it, so there is nothing to replay onto it.
    #[test]
    fn test_forgetting_clears_the_history_and_the_offset() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (_position, _physics, mut player) = run(&mut prediction, 40, moving(MovingType::MovingRight), &blocks, &liquids);

        let (mut server_position, server_physics) = prediction.state_at(20).unwrap();
        server_position.set_x(server_position.x() - Fixed::ONE);
        prediction.reconcile(20, server_position, server_physics, &mut player, &blocks, &liquids).unwrap();

        prediction.forget();

        assert_eq!(prediction.history_len(), 0);
        assert_eq!(prediction.visual_offset(), (Fixed::ZERO, Fixed::ZERO));
    }

    /// A correction far too large to slide across is shown as the jump it is. Sliding a
    /// player the width of the world would be worse than teleporting them.
    #[test]
    fn test_a_huge_correction_is_not_smoothed() {
        let (blocks, liquids) = world();
        let mut prediction = Prediction::new();
        let (_position, _physics, mut player) = run(&mut prediction, 40, moving(MovingType::Standing), &blocks, &liquids);

        let (mut server_position, server_physics) = prediction.state_at(20).unwrap();
        server_position.set_x(server_position.x() + Fixed::from_int(500));
        prediction.reconcile(20, server_position, server_physics, &mut player, &blocks, &liquids).unwrap();

        assert_eq!(prediction.visual_offset(), (Fixed::ZERO, Fixed::ZERO), "a teleport should look like a teleport");
    }
}
