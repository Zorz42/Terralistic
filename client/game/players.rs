use anyhow::{anyhow, Result};
use hecs::Entity;

use crate::client::game::camera::Camera;
use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::entities::{Entities, PhysicsComponent, PositionComponent};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use crate::shared::players::{
    spawn_player, update_players_ms, Direction, MovingType, PlayerComponent,
    PlayerMovingPacketToClient, PlayerMovingPacketToServer, PlayerSpawnPacket, PLAYER_HEIGHT,
    PLAYER_WIDTH,
};

pub struct ClientPlayers {
    main_player: Option<Entity>,
    main_player_name: String,
    player_texture: gfx::Texture,
}

impl ClientPlayers {
    pub fn new(player_name: &str) -> Self {
        Self {
            main_player: None,
            main_player_name: player_name.to_owned(),
            player_texture: gfx::Texture::new(),
        }
    }

    pub fn load_resources(&mut self, mods: &ModManager) -> Result<()> {
        let mut template_surface = gfx::Surface::deserialize_from_bytes(
            mods.get_resource("misc:skin_template.opa").ok_or_else(|| {
                anyhow::anyhow!("Failed to load misc:skin_template.opa from mod manager")
            })?,
        )?;

        let player_surface = gfx::Surface::deserialize_from_bytes(
            mods.get_resource("misc:skin.opa")
                .ok_or_else(|| anyhow::anyhow!("Failed to load misc:skin.opa from mod manager"))?,
        )?;

        for (_, color) in template_surface.iter_mut() {
            let x = color.r as i32 / 8;
            let y = color.g as i32 / 8;

            *color = *player_surface.get_pixel(gfx::IntPos(x, y))?;
        }

        self.player_texture = gfx::Texture::load_from_surface(&template_surface);

        Ok(())
    }

    fn send_moving_state(
        networking: &mut ClientNetworking,
        player_component: &PlayerComponent,
    ) -> Result<()> {
        let packet = Packet::new(PlayerMovingPacketToServer {
            moving_type: player_component.get_moving_type(),
            jumping: player_component.jumping,
        })?;

        networking.send_packet(packet)?;
        Ok(())
    }

    fn set_jumping(
        networking: &mut ClientNetworking,
        player_component: &mut PlayerComponent,
        jumping: bool,
    ) -> Result<()> {
        if jumping == player_component.jumping {
            return Ok(());
        }

        player_component.jumping = jumping;

        Self::send_moving_state(networking, player_component)?;

        Ok(())
    }

    fn set_moving_type(
        networking: &mut ClientNetworking,
        moving_type: MovingType,
        player_component: &mut PlayerComponent,
        physics: &mut PhysicsComponent,
    ) -> Result<()> {
        if moving_type == player_component.get_moving_type() {
            return Ok(());
        }

        player_component.set_moving_type(moving_type, physics);
        Self::send_moving_state(networking, player_component)?;

        Ok(())
    }

    pub fn update(
        &mut self,
        graphics: &gfx::GraphicsContext,
        entities: &mut Entities,
        networking: &mut ClientNetworking,
        blocks: &Blocks,
    ) -> Result<()> {
        if let Some(main_player) = self.main_player {
            if let Ok((physics, player_component)) = entities
                .ecs
                .query_one_mut::<(&mut PhysicsComponent, &mut PlayerComponent)>(main_player)
            {
                Self::set_jumping(
                    networking,
                    player_component,
                    graphics.renderer.get_key_state(gfx::Key::Space),
                )?;

                let key_a_pressed = graphics.renderer.get_key_state(gfx::Key::A);
                let key_d_pressed = graphics.renderer.get_key_state(gfx::Key::D);

                let moving_type = match (key_a_pressed, key_d_pressed) {
                    (true, false) => MovingType::MovingLeft,
                    (false, true) => MovingType::MovingRight,
                    _ => MovingType::Standing,
                };

                Self::set_moving_type(networking, moving_type, player_component, physics)?;
            }
        }

        update_players_ms(entities, blocks);

        Ok(())
    }

    pub fn render(
        &self,
        graphics: &gfx::GraphicsContext,
        entities: &mut Entities,
        camera: &Camera,
    ) {
        for (_, (position, player_component)) in entities
            .ecs
            .query_mut::<(&PositionComponent, &PlayerComponent)>()
        {
            let x = position.x() * RENDER_BLOCK_WIDTH
                - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
            let y = position.y() * RENDER_BLOCK_WIDTH
                - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;

            let src_rect = gfx::Rect::new(
                gfx::FloatPos(
                    player_component.animation_frame as f32 * PLAYER_WIDTH * BLOCK_WIDTH,
                    0.0,
                ),
                gfx::FloatSize(PLAYER_WIDTH * BLOCK_WIDTH, PLAYER_HEIGHT * BLOCK_WIDTH),
            );

            let flipped = match player_component.direction {
                Direction::Left => true,
                Direction::Right => false,
            };

            self.player_texture.render(
                &graphics.renderer,
                RENDER_SCALE,
                gfx::FloatPos(x.round(), y.round()),
                Some(src_rect),
                flipped,
                None,
            );
        }
    }

    pub fn on_event(&mut self, event: &Event, entities: &mut Entities) -> Result<()> {
        if let Some(packet_event) = event.downcast::<Packet>() {
            if let Some(packet) = packet_event.try_deserialize::<PlayerSpawnPacket>() {
                let player = spawn_player(entities, packet.x, packet.y, &packet.name, packet.id)?;
                if packet.name == self.main_player_name {
                    self.main_player = Some(player);
                }
            }
            if let Some(packet) = packet_event.try_deserialize::<PlayerMovingPacketToClient>() {
                let entity = entities.get_entity_from_id(packet.player_id)?;
                let mut physics_component = entities
                    .ecs
                    .query_one::<&mut PhysicsComponent>(entity)?
                    .get()
                    .ok_or_else(|| anyhow!("unwrap failed"))?
                    .clone();
                {
                    let player_component =
                        entities.ecs.query_one_mut::<&mut PlayerComponent>(entity)?;
                    player_component.set_moving_type(packet.moving_type, &mut physics_component);
                    player_component.jumping = packet.jumping;
                }

                entities.ecs.insert_one(entity, physics_component)?;
            }
        }
        Ok(())
    }

    pub const fn get_main_player(&self) -> Option<Entity> {
        self.main_player
    }
}
