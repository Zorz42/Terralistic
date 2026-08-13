use crate::libraries::graphics as gfx;

use super::draw_list::DrawTarget;

/// Half the width of the gaussian falloff, and the size of a corner piece.
const FADE: f32 = 200.0;
/// The size of the baked texture: an opaque core with a `FADE` wide falloff on each side.
const TEXTURE_SIZE: f32 = 700.0;
/// How far a piece may reach along an edge before the middle has to be tiled instead.
const MAX_EDGE: f32 = 350.0;

/// Draws a soft drop shadow around a rectangle, from a gaussian baked into a texture once.
pub struct ShadowContext {
    pub shadow_texture: gfx::Texture,
}

impl ShadowContext {
    /// Multiplies a pixel's alpha by a gaussian `h` pixels into the falloff.
    fn fade_pixel(h: i32, pixel: &mut gfx::Color) {
        let alpha = std::f32::consts::E.powf(-((h * h) as f32 / 2000.0));
        let prev_alpha = pixel.a as f32 / 255.0;
        *pixel = gfx::Color::new(0, 0, 0, (alpha * prev_alpha * 255.0) as u8);
    }

    /// Bakes a black square whose edges fade out along a gaussian: opaque in the middle
    /// 300x300, falling off over the 200px border on each side. `render` then draws the eight
    /// pieces of that border around whatever rectangle wants a shadow.
    pub fn new() -> Self {
        let mut surface = gfx::Surface::new(gfx::IntSize(TEXTURE_SIZE as u32, TEXTURE_SIZE as u32));
        let fade = FADE as i32;
        let far = (TEXTURE_SIZE - FADE) as i32;
        for (pos, pixel) in surface.iter_mut() {
            // One pass, not two: filling with opaque black and fading it are the same loop.
            *pixel = gfx::Color::new(0, 0, 0, 255);

            if pos.1 < fade {
                Self::fade_pixel(fade - pos.1, pixel);
            }
            if pos.0 < fade {
                Self::fade_pixel(fade - pos.0, pixel);
            }
            if pos.1 > far {
                Self::fade_pixel(pos.1 - far, pixel);
            }
            if pos.0 > far {
                Self::fade_pixel(pos.0 - far, pixel);
            }
        }

        Self {
            shadow_texture: gfx::Texture::load_from_surface(&surface),
        }
    }

    /// Renders the shadow around `rect`.
    ///
    /// Nothing but nearest-neighbour texture draws - the gaussian was baked in `new` and no
    /// shader is involved, which is why the shadow's golden images can be exact.
    pub fn render(&self, target: &dyn DrawTarget, rect: &gfx::Rect, shadow_intensity: f32) {
        let color = gfx::Color::new(0, 0, 0, (80.0 * shadow_intensity) as u8);
        // A piece may cover at most half the rectangle plus the falloff, so that opposite
        // corners meet in the middle rather than overlapping.
        let edge_width = f32::min(FADE + rect.size.0 / 2.0, MAX_EDGE);
        let edge_height = f32::min(FADE + rect.size.1 / 2.0, MAX_EDGE);

        // (offset from the rectangle's top left, region of the texture).
        let mut pieces = vec![
            (gfx::FloatPos(-FADE, -FADE), gfx::FloatPos(0.0, 0.0), gfx::FloatSize(edge_width, FADE)),
            (gfx::FloatPos(-FADE, 0.0), gfx::FloatPos(0.0, FADE), gfx::FloatSize(FADE, edge_height - FADE)),
            (
                gfx::FloatPos(rect.size.0 - edge_width + FADE, -FADE),
                gfx::FloatPos(TEXTURE_SIZE - edge_width, 0.0),
                gfx::FloatSize(edge_width, FADE),
            ),
            (gfx::FloatPos(rect.size.0, 0.0), gfx::FloatPos(TEXTURE_SIZE - FADE, FADE), gfx::FloatSize(FADE, edge_height - FADE)),
            (
                gfx::FloatPos(-FADE, rect.size.1 - edge_height + FADE),
                gfx::FloatPos(0.0, TEXTURE_SIZE - edge_height),
                gfx::FloatSize(FADE, edge_height - FADE),
            ),
            (gfx::FloatPos(-FADE, rect.size.1), gfx::FloatPos(0.0, TEXTURE_SIZE - FADE), gfx::FloatSize(edge_width, FADE)),
            (
                gfx::FloatPos(rect.size.0, rect.size.1 - edge_height + FADE),
                gfx::FloatPos(TEXTURE_SIZE - FADE, TEXTURE_SIZE - edge_height),
                gfx::FloatSize(FADE, edge_height - FADE),
            ),
            (
                gfx::FloatPos(rect.size.0 - edge_width + FADE, rect.size.1),
                gfx::FloatPos(TEXTURE_SIZE - edge_width, TEXTURE_SIZE - FADE),
                gfx::FloatSize(edge_width, FADE),
            ),
        ];

        // A rectangle too tall or too wide for the pieces above to meet gets the middle of the
        // texture tiled along the gap, 100px at a time.
        if (edge_height - MAX_EDGE).abs() < f32::EPSILON {
            let mut left = rect.size.1 - 300.0;
            while left > 0.0 {
                let y = rect.size.1 - 150.0 - left;
                let size = gfx::FloatSize(FADE, f32::min(100.0, left));
                pieces.push((gfx::FloatPos(-FADE, y), gfx::FloatPos(0.0, 300.0), size));
                pieces.push((gfx::FloatPos(rect.size.0, y), gfx::FloatPos(TEXTURE_SIZE - FADE, 300.0), size));
                left -= 100.0;
            }
        }

        if (edge_width - MAX_EDGE).abs() < f32::EPSILON {
            let mut left = rect.size.0 - 300.0;
            while left > 0.0 {
                let x = rect.size.0 - 150.0 - left;
                let size = gfx::FloatSize(f32::min(100.0, left), FADE);
                pieces.push((gfx::FloatPos(x, -FADE), gfx::FloatPos(300.0, 0.0), size));
                pieces.push((gfx::FloatPos(x, rect.size.1), gfx::FloatPos(300.0, TEXTURE_SIZE - FADE), size));
                left -= 100.0;
            }
        }

        for (offset, src_pos, src_size) in pieces {
            self.shadow_texture.render(target, 1.0, rect.pos + offset, Some(gfx::Rect::new(src_pos, src_size)), false, Some(color));
        }
    }
}
