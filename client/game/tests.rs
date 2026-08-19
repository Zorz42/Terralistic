#![allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use crate::client::game::entities::ClientEntities;
    use crate::libraries::fixed::Fixed;
    use crate::shared::entities::{EntityId, EntityState, PhysicsComponent, PositionComponent};

    const ID: EntityId = EntityId::from_raw(1);

    /// One entity at `x`, moving right at one block per tick, with the id the server names it by.
    fn moving_entity(client: &ClientEntities, id: EntityId, x: i32) {
        let mut entities = client.get_entities();
        let mut physics = PhysicsComponent::new(Fixed::ONE, Fixed::ONE);
        physics.velocity_x = Fixed::from_int(200);
        physics.acceleration_y = Fixed::ZERO;
        let entity = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(x), Fixed::ZERO), physics));
        entities.assign_id(entity, id).unwrap();
    }

    fn position_of(client: &ClientEntities, id: EntityId) -> PositionComponent {
        let mut entities = client.get_entities();
        let entity = entities.get_entity_from_id(id).unwrap();
        *entities.ecs.query_one_mut::<&PositionComponent>(entity).unwrap()
    }

    /// Advances the entity by hand, one block a tick, recording each tick as the real loop does.
    fn run(client: &mut ClientEntities, id: EntityId, from_tick: u64, ticks: u64) {
        for tick in from_tick..from_tick + ticks {
            {
                let mut entities = client.get_entities();
                let entity = entities.get_entity_from_id(id).unwrap();
                let position = entities.ecs.query_one_mut::<&mut PositionComponent>(entity).unwrap();
                position.set_x(position.x() + Fixed::ONE);
            }
            client.record_tick(tick);
        }
    }

    /// **The regression test for items trailing behind the player.**
    ///
    /// A snapshot names the tick it is for, and by the time it lands this client has simulated
    /// further. Measuring it against where the entity is *now* measures how far it moved while
    /// the packet was in flight, and drags it back by that much on every snapshot - which is
    /// exactly an item accelerating toward a player faster than the sync rate. Measured
    /// against the tick it is for, a client that agreed is not moved at all.
    #[test]
    fn test_a_stale_snapshot_this_client_agreed_with_moves_nothing() {
        let mut client = ClientEntities::new();
        moving_entity(&client, ID, 0);
        run(&mut client, ID, 1, 40);

        let agreed = position_of(&client, ID);
        let at_tick_20 = Fixed::from_int(20);

        client
            .apply_state(
                20,
                &EntityState {
                    id: ID,
                    x: at_tick_20,
                    y: Fixed::ZERO,
                    velocity_x: Fixed::from_int(200),
                    velocity_y: Fixed::ZERO,
                },
            )
            .unwrap();

        assert_eq!(position_of(&client, ID).x(), agreed.x(), "a snapshot the client already agreed with is not a correction");
    }

    /// A real disagreement is still corrected - and by the difference at the tick it was
    /// measured on, applied to where the entity is now, so the ticks since are kept.
    #[test]
    fn test_a_real_disagreement_is_corrected_by_its_own_size() {
        let mut client = ClientEntities::new();
        moving_entity(&client, ID, 0);
        run(&mut client, ID, 1, 40);

        let before = position_of(&client, ID).x();

        client
            .apply_state(
                20,
                &EntityState {
                    id: ID,
                    // two blocks right of where this client had it on tick 20
                    x: Fixed::from_int(22),
                    y: Fixed::ZERO,
                    velocity_x: Fixed::from_int(200),
                    velocity_y: Fixed::ZERO,
                },
            )
            .unwrap();

        // half of the two block error, the rest left for the next snapshot
        assert_eq!(position_of(&client, ID).x(), before + Fixed::ONE);
    }

    /// A snapshot for a tick this client has no record of has nothing to take a difference
    /// against, so the server's answer is taken as given rather than guessed at.
    #[test]
    fn test_a_snapshot_for_an_unknown_tick_is_taken_whole() {
        let mut client = ClientEntities::new();
        moving_entity(&client, ID, 0);
        run(&mut client, ID, 1, 5);

        client
            .apply_state(
                9999,
                &EntityState {
                    id: ID,
                    x: Fixed::from_int(700),
                    y: Fixed::ZERO,
                    velocity_x: Fixed::ZERO,
                    velocity_y: Fixed::ZERO,
                },
            )
            .unwrap();

        assert_eq!(position_of(&client, ID).x(), Fixed::from_int(700));
    }

    /// History is dropped with the entity it belongs to, or every item ever picked up stays
    /// in the map for the session.
    #[test]
    fn test_history_does_not_outlive_the_entity() {
        let mut client = ClientEntities::new();
        moving_entity(&client, ID, 0);
        run(&mut client, ID, 1, 3);
        assert_eq!(client.history_len(), 1);

        {
            let mut entities = client.get_entities();
            let entity = entities.get_entity_from_id(ID).unwrap();
            entities.ecs.despawn(entity).unwrap();
        }
        client.record_tick(4);

        assert_eq!(client.history_len(), 0);
    }
}
