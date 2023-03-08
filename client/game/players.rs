use crate::client::game::camera::Camera;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize};
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::entities::{reduce_by, Entities, PhysicsComponent, PositionComponent, is_touching_ground};
use crate::shared::players::{
    spawn_player, PlayerComponent, PLAYER_ACCELERATION, PLAYER_HEIGHT, PLAYER_INITIAL_SPEED,
    PLAYER_JUMP_SPEED, PLAYER_WIDTH,
};
use hecs::Entity;

pub struct ClientPlayers {
    main_player: Option<Entity>,
}

impl ClientPlayers {
    pub const fn new() -> Self {
        Self { main_player: None }
    }

    pub fn init(&mut self, entities: &mut Entities, blocks: &Blocks) {
        self.main_player = Some(spawn_player(
            entities,
            blocks.get_width() as f32 / 2.0,
            0.0,
            Some(100),
        ));
    }

    pub fn render(
        &self,
        graphics: &mut gfx::GraphicsContext,
        entities: &mut Entities,
        camera: &Camera,
        blocks: &Blocks,
    ) {
        if let Some(main_player) = self.main_player {
            if let Ok((position, physics)) = entities.ecs.query_one_mut::<(&mut PositionComponent, &mut PhysicsComponent)>(main_player) {
                if graphics.renderer.get_key_state(gfx::Key::Space) && is_touching_ground(position, physics, blocks) {
                    physics.increase_velocity_y(-PLAYER_JUMP_SPEED);
                }
            }
        }

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
        if let Some(main_player) = self.main_player {
            if let Ok(mut physics) = entities.ecs.get::<&mut PhysicsComponent>(main_player) {
                if let Some(event) = event.downcast::<gfx::Event>() {
                    match event {
                        gfx::Event::KeyPress(key, false) => match key {
                            gfx::Key::A => {
                                physics.increase_acceleration_x(-PLAYER_ACCELERATION);
                                physics.increase_velocity_x(-PLAYER_INITIAL_SPEED);
                            }
                            gfx::Key::D => {
                                physics.increase_acceleration_x(PLAYER_ACCELERATION);
                                physics.increase_velocity_x(PLAYER_INITIAL_SPEED);
                            }
                            _ => {}
                        },
                        gfx::Event::KeyRelease(key, false) => match key {
                            gfx::Key::A => {
                                physics.increase_acceleration_x(PLAYER_ACCELERATION);
                                let mut velocity = physics.get_velocity_x();
                                reduce_by(&mut velocity, PLAYER_INITIAL_SPEED);
                                physics.set_velocity_x(velocity);
                            }
                            gfx::Key::D => {
                                physics.increase_acceleration_x(-PLAYER_ACCELERATION);
                                let mut velocity = physics.get_velocity_x();
                                reduce_by(&mut velocity, PLAYER_INITIAL_SPEED);
                                physics.set_velocity_x(velocity);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    pub const fn get_main_player(&self) -> Option<Entity> {
        self.main_player
    }
}
