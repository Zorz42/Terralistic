use hecs::Entity;

use crate::client::game::camera::Camera;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize};
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::entities::{Entities, PhysicsComponent, PositionComponent};
use crate::shared::players::{
    spawn_player, update_player, MovingType, PlayerComponent, PlayerMovingComponent, PLAYER_HEIGHT,
    PLAYER_WIDTH,
};

pub struct ClientPlayers {
    main_player: Option<Entity>,
    main_player_moving: PlayerMovingComponent,
}

impl ClientPlayers {
    pub const fn new() -> Self {
        Self {
            main_player: None,
            main_player_moving: PlayerMovingComponent::new(),
        }
    }

    pub fn init(&mut self, entities: &mut Entities, blocks: &Blocks) {
        self.main_player = Some(spawn_player(
            entities,
            blocks.get_width() as f32 / 2.0,
            0.0,
            Some(100),
        ));
    }

    pub fn update(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        entities: &mut Entities,
        blocks: &Blocks,
    ) {
        self.main_player_moving.jumping = graphics.renderer.get_key_state(gfx::Key::Space);
        if let Some(main_player) = self.main_player {
            if let Ok((position, physics)) = entities
                .ecs
                .query_one_mut::<(&mut PositionComponent, &mut PhysicsComponent)>(main_player)
            {
                update_player(position, physics, &self.main_player_moving, blocks);

                let key_a_pressed = graphics.renderer.get_key_state(gfx::Key::A);
                let key_d_pressed = graphics.renderer.get_key_state(gfx::Key::D);

                let moving_type = match (key_a_pressed, key_d_pressed) {
                    (true, false) => MovingType::MovingLeft,
                    (false, true) => MovingType::MovingRight,
                    _ => MovingType::Standing,
                };

                self.main_player_moving
                    .set_moving_type(moving_type, physics);
            }
        }
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

    pub const fn get_main_player(&self) -> Option<Entity> {
        self.main_player
    }
}
