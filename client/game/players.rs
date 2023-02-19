use crate::client::game::camera::Camera;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize};
use crate::shared::blocks::{RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::entities::{Entities, PositionComponent};
use crate::shared::players::{PlayerComponent, PLAYER_HEIGHT, PLAYER_WIDTH};

pub fn render_players(
    graphics: &mut gfx::GraphicsContext,
    entities: &mut Entities,
    camera: &Camera,
) {
    for (_entity, (position, _player_component)) in entities
        .ecs
        .query_mut::<(&PositionComponent, &PlayerComponent)>()
    {
        let x = position.x() * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_SCALE;
        let y = position.y() * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_SCALE;

        gfx::Rect::new(
            FloatPos(x.round(), y.round()),
            FloatSize(
                PLAYER_WIDTH * RENDER_BLOCK_WIDTH,
                PLAYER_HEIGHT * RENDER_BLOCK_WIDTH,
            ),
        )
        .render(graphics, gfx::Color::new(0, 0, 255, 255));
    }
}
