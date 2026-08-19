use anyhow::Result;
use hecs::Entity;

use crate::client::game::camera::Camera;
use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::scripting::ScriptHost;
use crate::libraries::ui::UiContext;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::entities::{Entities, EntityDespawnEvent, HealthComponent, PhysicsComponent, PositionComponent};
use crate::shared::liquids::Liquids;
use crate::shared::packet::Packet;
use crate::shared::players::{
    spawn_player, update_players_ms, Direction, MovingType, PlayerComponent, PlayerInput, PlayerInputPacket, PlayerInputPacketToClient, PlayerSpawnPacket, PLAYER_HEIGHT, PLAYER_MAX_HEALTH,
    PLAYER_WIDTH,
};

pub struct ClientPlayers {
    main_player: Option<Entity>,
    main_player_name: String,
    player_texture: gfx::Texture,
    waiting_for_player: bool,
    pub controls_enabled: bool,
}

impl ClientPlayers {
    pub fn new(player_name: &str) -> Self {
        Self {
            main_player: None,
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

    pub fn render(&self, graphics: &gfx::GraphicsContext, entities: &mut Entities, camera: &Camera) {
        for (position, player_component) in entities.ecs.query_mut::<(&PositionComponent, &PlayerComponent)>() {
            let x = position.x().to_f32() * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
            let y = position.y().to_f32() * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;

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

    pub fn on_event(&mut self, event: &Event, entities: &mut Entities) -> Result<()> {
        if let Some(packet_event) = event.downcast::<Packet>() {
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

    pub const fn get_main_player(&self) -> Option<Entity> {
        self.main_player
    }

    pub const fn is_waiting_for_player(&self) -> bool {
        self.waiting_for_player
    }
}
