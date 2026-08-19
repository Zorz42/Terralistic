use anyhow::Result;
use hecs::Entity;

use crate::client::game::camera::Camera;
use crate::client::game::networking::ClientNetworking;
use crate::client::game::prediction::Prediction;
use crate::libraries::events::Event;
use crate::libraries::fixed::Fixed;
use crate::libraries::graphics as gfx;
use crate::libraries::scripting::ScriptHost;
use crate::libraries::ui::UiContext;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::entities::{state_hash, Entities, EntityDespawnEvent, EntityState, EntitySyncPacket, HealthComponent, PhysicsComponent, PositionComponent};
use crate::shared::liquids::Liquids;
use crate::shared::packet::Packet;
use crate::shared::players::{
    spawn_player, update_players_ms, Direction, MovingType, PlayerComponent, PlayerInput, PlayerInputPacket, PlayerInputPacketToClient, PlayerSpawnPacket, PlayerStateHashPacket, PLAYER_HEIGHT,
    PLAYER_MAX_HEALTH, PLAYER_WIDTH,
};
use crate::shared::STATE_CHECK_INTERVAL_TICKS;

pub struct ClientPlayers {
    main_player: Option<Entity>,
    prediction: Prediction,
    main_player_name: String,
    player_texture: gfx::Texture,
    waiting_for_player: bool,
    pub controls_enabled: bool,
}

impl ClientPlayers {
    pub fn new(player_name: &str) -> Self {
        Self {
            main_player: None,
            prediction: Prediction::new(),
            main_player_name: player_name.to_owned(),
            player_texture: gfx::Texture::new(),
            controls_enabled: true,
            waiting_for_player: true,
        }
    }

    pub fn load_resources(&mut self, mods: &ScriptHost) -> Result<()> {
        let mut template_surface = gfx::Surface::deserialize_from_bytes(
            mods.get_resource("misc:skin_template.opa")
                .ok_or_else(|| anyhow::anyhow!("Failed to load misc:skin_template.opa from mod manager"))?,
        )?;

        let player_surface = gfx::Surface::deserialize_from_bytes(mods.get_resource("misc:skin.opa").ok_or_else(|| anyhow::anyhow!("Failed to load misc:skin.opa from mod manager"))?)?;

        for (_, color) in template_surface.iter_mut() {
            let x = color.r as i32 / 8;
            let y = color.g as i32 / 8;

            *color = *player_surface.get_pixel(gfx::IntPos(x, y))?;
        }

        self.player_texture = gfx::Texture::load_from_surface(&template_surface);

        Ok(())
    }

    /// Reads the keys, applies them to this client's own player, and tells the server what
    /// they were and which tick they were for.
    ///
    /// The input is sent only when it changes: it is a held state that the server keeps in
    /// force, so walking across the world is two packets rather than one per tick. The tick
    /// is what makes that safe - the server applies it at the moment it was meant for
    /// instead of whenever the packet happened to land.
    pub fn update(&self, tick: u64, graphics: &gfx::GraphicsContext, entities: &mut Entities, networking: &mut ClientNetworking, blocks: &Blocks, liquids: &Liquids) -> Result<()> {
        if let Some(main_player) = self.main_player {
            let input = PlayerInput {
                moving_type: match (
                    graphics.get_key_state(gfx::Key::A) && self.controls_enabled,
                    graphics.get_key_state(gfx::Key::D) && self.controls_enabled,
                ) {
                    (true, false) => MovingType::MovingLeft,
                    (false, true) => MovingType::MovingRight,
                    _ => MovingType::Standing,
                },
                jumping: graphics.get_key_state(gfx::Key::Space) && self.controls_enabled,
            };

            if let Ok((physics, player_component)) = entities.ecs.query_one_mut::<(&mut PhysicsComponent, &mut PlayerComponent)>(main_player) {
                if player_component.get_input() != input {
                    player_component.apply_input(input, physics);
                    networking.send_packet(Packet::new(PlayerInputPacket { tick, input })?)?;
                }
            }
        }

        update_players_ms(entities, blocks, liquids);

        Ok(())
    }

    /// Remembers where a tick left this client's player, so a server state for the same tick
    /// can be compared against what the client actually had rather than snapped to blind.
    ///
    /// **Called after the physics step, not inside `update`.** The server's state for a tick
    /// is what it holds once that tick is fully simulated; a frame recorded a step earlier
    /// would differ from it every single time, and every sync would look like a correction.
    pub fn record_tick(&mut self, tick: u64, entities: &mut Entities, networking: &mut ClientNetworking) -> Result<()> {
        if let Some(main_player) = self.main_player {
            if let Ok((position, physics, player)) = entities.ecs.query_one_mut::<(&PositionComponent, &PhysicsComponent, &PlayerComponent)>(main_player) {
                self.prediction.record(tick, player.get_input(), *position, *physics);

                // The two sides run the same inputs through the same deterministic step, so
                // these must agree exactly and forever. Sending one now and then is what
                // turns "it sometimes pulls me backwards" into a tick number to look at.
                if tick.is_multiple_of(STATE_CHECK_INTERVAL_TICKS) {
                    let hash = state_hash(position, physics);
                    networking.send_packet(Packet::new(PlayerStateHashPacket { tick, hash })?)?;
                }
            }
        }
        self.prediction.decay_visual_error();

        Ok(())
    }

    pub fn render(&self, graphics: &gfx::GraphicsContext, entities: &mut Entities, camera: &Camera) {
        for (entity, position, player_component) in entities.ecs.query_mut::<(Entity, &PositionComponent, &PlayerComponent)>() {
            let offset = self.draw_offset(entity);
            let x = (position.x() + offset.0).to_f32() * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
            let y = (position.y() + offset.1).to_f32() * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;

            let src_rect = gfx::Rect::new(
                gfx::FloatPos(player_component.animation_frame as f32 * PLAYER_WIDTH.to_f32() * BLOCK_WIDTH, 0.0),
                gfx::FloatSize(PLAYER_WIDTH.to_f32() * BLOCK_WIDTH, PLAYER_HEIGHT.to_f32() * BLOCK_WIDTH),
            );

            let flipped = match player_component.direction {
                Direction::Left => true,
                Direction::Right => false,
            };

            self.player_texture.render(graphics, RENDER_SCALE, gfx::FloatPos(x.round(), y.round()), Some(src_rect), flipped, None);
        }
    }

    pub fn on_event(&mut self, event: &Event, entities: &mut Entities, blocks: &Blocks, liquids: &Liquids) -> Result<()> {
        if let Some(packet_event) = event.downcast::<Packet>() {
            if let Some(packet) = packet_event.try_deserialize::<EntitySyncPacket>() {
                if let Some(main_player) = self.main_player {
                    let mine = packet.entities.iter().find(|state| entities.get_entity_from_id(state.id).is_ok_and(|entity| entity == main_player));
                    if let Some(state) = mine {
                        self.reconcile_main_player(packet.tick, state, entities, blocks, liquids)?;
                    }
                }
            }
            if let Some(packet) = packet_event.try_deserialize::<PlayerSpawnPacket>() {
                let player = spawn_player(entities, packet.x, packet.y, &packet.name, packet.id, HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH))?;
                if packet.name == self.main_player_name {
                    self.main_player = Some(player);
                    self.waiting_for_player = false;
                }
            } else if let Some(packet) = packet_event.try_deserialize::<PlayerInputPacketToClient>() {
                // another player's controls. Applied on arrival rather than at `packet.tick`:
                // this client does not predict a remote player, so there is nothing to rewind,
                // and the difference is the fraction of a second the input spent in flight.
                let entity = entities.get_entity_from_id(packet.player_id)?;
                let mut physics_component = *entities.ecs.query_one::<&mut PhysicsComponent>(entity).get()?;
                {
                    let player_component = entities.ecs.query_one_mut::<&mut PlayerComponent>(entity)?;
                    player_component.apply_input(packet.input, &mut physics_component);
                }

                entities.ecs.insert_one(entity, physics_component)?;
            }
        }

        if let Some(despawn_event) = event.downcast::<EntityDespawnEvent>() {
            if let Some(main_player) = self.main_player {
                let id = entities.get_id_from_entity(main_player)?;
                if id == despawn_event.id {
                    self.main_player = None;
                }
            }
        }

        Ok(())
    }

    /// Checks the server's answer for one tick against what this client had, and rewinds and
    /// replays if they differ.
    ///
    /// A forced state - a spawn, a respawn, a teleport - is the server deciding where the
    /// player is rather than reporting where the simulation put it, so it is taken as given
    /// and the history is dropped: there is nothing to replay onto a decision.
    fn reconcile_main_player(&mut self, tick: u64, state: &EntityState, entities: &mut Entities, blocks: &Blocks, liquids: &Liquids) -> Result<()> {
        let Some(main_player) = self.main_player else { return Ok(()) };

        let (position_component, physics_component, player_component) = entities.ecs.query_one_mut::<(&mut PositionComponent, &mut PhysicsComponent, &mut PlayerComponent)>(main_player)?;

        let server_position = PositionComponent::new(state.x, state.y);
        let mut server_physics = *physics_component;
        server_physics.velocity_x = state.velocity_x;
        server_physics.velocity_y = state.velocity_y;

        if let Some((position, physics)) = self.prediction.reconcile(tick, server_position, server_physics, player_component, blocks, liquids) {
            *position_component = position;
            *physics_component = physics;
        }

        Ok(())
    }

    /// Where to draw the main player, which trails a correction the simulation already took.
    fn draw_offset(&self, entity: Entity) -> (Fixed, Fixed) {
        if Some(entity) == self.main_player {
            self.prediction.visual_offset()
        } else {
            (Fixed::ZERO, Fixed::ZERO)
        }
    }

    #[must_use]
    pub const fn corrections(&self) -> u64 {
        self.prediction.corrections
    }

    pub const fn get_main_player(&self) -> Option<Entity> {
        self.main_player
    }

    pub const fn is_waiting_for_player(&self) -> bool {
        self.waiting_for_player
    }
}
