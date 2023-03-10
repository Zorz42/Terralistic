use anyhow::Result;
use hecs::Entity;

use crate::client::game::camera::Camera;
use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize};
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::entities::{Entities, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;
use crate::shared::players::{
    spawn_player, update_player, update_players, MovingType, PlayerComponent, PlayerMovingPacket,
    PlayerSpawnPacket, PLAYER_HEIGHT, PLAYER_WIDTH,
};

pub struct ClientPlayers {
    main_player: Option<Entity>,
    main_player_name: String,
}

impl ClientPlayers {
    pub fn new(player_name: &str) -> Self {
        Self {
            main_player: None,
            main_player_name: player_name.to_owned(),
        }
    }

    fn send_moving_state(
        &self,
        networking: &mut ClientNetworking,
        player_component: &PlayerComponent,
    ) -> Result<()> {
        let packet = Packet::new(PlayerMovingPacket {
            moving_type: player_component.get_moving_type(),
            jumping: player_component.jumping,
            player_id: 0,
        })?;

        networking.send_packet(&packet)?;
        Ok(())
    }

    fn set_jumping(
        &mut self,
        networking: &mut ClientNetworking,
        player_component: &mut PlayerComponent,
        jumping: bool,
    ) -> Result<()> {
        if jumping == player_component.jumping {
            return Ok(());
        }

        player_component.jumping = jumping;

        self.send_moving_state(networking, player_component)?;

        Ok(())
    }

    fn set_moving_type(
        &mut self,
        networking: &mut ClientNetworking,
        moving_type: MovingType,
        player_component: &mut PlayerComponent,
        physics: &mut PhysicsComponent,
    ) -> Result<()> {
        if moving_type == player_component.get_moving_type() {
            return Ok(());
        }

        player_component.set_moving_type(moving_type, physics);
        self.send_moving_state(networking, player_component)?;

        Ok(())
    }

    pub fn update(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        entities: &mut Entities,
        networking: &mut ClientNetworking,
        blocks: &Blocks,
    ) -> Result<()> {
        if let Some(main_player) = self.main_player {
            if let Ok((position, physics, player_component)) =
                entities.ecs.query_one_mut::<(
                    &mut PositionComponent,
                    &mut PhysicsComponent,
                    &mut PlayerComponent,
                )>(main_player)
            {
                self.set_jumping(
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

                self.set_moving_type(networking, moving_type, player_component, physics)?;
            }
        }

        update_players(entities, blocks);

        Ok(())
    }

    pub fn render(
        &self,
        graphics: &mut gfx::GraphicsContext,
        entities: &mut Entities,
        camera: &Camera,
    ) {
        for (entity, (position, _player_component)) in entities
            .ecs
            .query_mut::<(&PositionComponent, &PlayerComponent)>()
        {
            let x = position.x() * RENDER_BLOCK_WIDTH
                - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
            let y = position.y() * RENDER_BLOCK_WIDTH
                - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;

            let color = self.main_player.map_or_else(
                || gfx::Color::new(100, 200, 100, 255),
                |player| {
                    if entity == player {
                        gfx::Color::new(100, 100, 200, 255)
                    } else {
                        gfx::Color::new(200, 100, 100, 255)
                    }
                },
            );

            gfx::Rect::new(
                FloatPos(x.round(), y.round()),
                FloatSize(
                    PLAYER_WIDTH * RENDER_BLOCK_WIDTH,
                    PLAYER_HEIGHT * RENDER_BLOCK_WIDTH,
                ),
            )
            .render(graphics, color);
        }
    }

    pub fn on_event(&mut self, event: &Event, entities: &mut Entities) {
        if let Some(packet_event) = event.downcast::<Packet>() {
            if let Some(packet) = packet_event.try_deserialize::<PlayerSpawnPacket>() {
                let player =
                    spawn_player(entities, packet.x, packet.y, &packet.name, Some(packet.id));
                if packet.name == self.main_player_name {
                    self.main_player = Some(player);
                }
            }
        }
    }

    pub const fn get_main_player(&self) -> Option<Entity> {
        self.main_player
    }
}
