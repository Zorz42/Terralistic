use super::{Color, GraphicsContext, Rect, Surface, Texture};
use anyhow::Result;

/// `ShadowContext` is a struct that contains the information needed to draw a shadow.
pub struct ShadowContext {
    pub shadow_texture: Texture,
}

impl ShadowContext {
    /// Creates a new ShadowContext.
    pub fn new() -> Self {
        let mut surface = Surface::new(700, 700);
        // fill surface with black color
        for x in 0..surface.get_width() {
            for y in 0..surface.get_height() {
                surface
                    .set_pixel(x as i32, y as i32, Color::new(0, 0, 0, 255))
                    .unwrap();
            }
        }

        // draw a shadow to the surface with center rectangle being transparent with
        // size 300, 300 and offset 200, 200. The blur around the rectangle is a gaussian
        // curve.
        let mut set_gaussian_pixel = |h, x, y| -> Result<()> {
            let alpha = core::f32::consts::E.powf(-((h * h) as f32 / 2000.0));
            let prev_alpha = surface.get_pixel(x, y)?.a as f32 / 255.0;
            surface.set_pixel(
                x,
                y,
                Color::new(0, 0, 0, (alpha * prev_alpha * 255.0) as u8),
            )
        };

        for x in 0..700 {
            for y in 0..200 {
                let h = 200 - y;
                set_gaussian_pixel(h, x, y);
            }
        }

        for x in 0..200 {
            for y in 0..700 {
                let h = 200 - x;
                set_gaussian_pixel(h, x, y);
            }
        }

        for x in 0..700 {
            for y in 500..700 {
                let h = y - 500;
                set_gaussian_pixel(h, x, y);
            }
        }

        for x in 500..700 {
            for y in 0..700 {
                let h = x - 500;
                set_gaussian_pixel(h, x, y);
            }
        }

        let shadow_texture = Texture::load_from_surface(&surface);

        Self { shadow_texture }
    }

    /// Renders the shadow.
    pub fn render(&self, graphics: &GraphicsContext, rect: &Rect, shadow_intensity: f32) {
        let shadow_color = Color::new(0, 0, 0, (80.0 * shadow_intensity) as u8);

        let shadow_edge_width = f32::min(200.0 + rect.w as f32 / 2.0, 350.0);
        let shadow_edge_height = f32::min(200.0 + rect.h as f32 / 2.0, 350.0);

        let wc = shadow_edge_width.ceil() as i32;
        let wf = shadow_edge_width.floor() as i32;
        let hc = shadow_edge_height.ceil() as i32;
        let hf = shadow_edge_height.floor() as i32;
        let mut elements = vec![
            ((rect.x - 200, rect.y - 200), Rect::new(0, 0, wf, 200)),
            ((rect.x - 200, rect.y), Rect::new(0, 200, 200, hc - 200)),
            (
                (rect.x + rect.w - wc + 200, rect.y - 200),
                Rect::new(700 - wc, 0, wc, 200),
            ),
            (
                (rect.x + rect.w, rect.y),
                Rect::new(500, 200, 200, hc - 200),
            ),
            (
                (rect.x - 200, rect.y + rect.h - hf + 200),
                Rect::new(0, 700 - hf, 200, hf - 200),
            ),
            ((rect.x - 200, rect.y + rect.h), Rect::new(0, 500, wf, 200)),
            (
                (rect.x + rect.w, rect.y + rect.h - hf + 200),
                Rect::new(500, 700 - hf, 200, hf - 200),
            ),
            (
                (rect.x + rect.w - wc + 200, rect.y + rect.h),
                Rect::new(700 - wc, 500, wc, 200),
            ),
        ];

        if shadow_edge_height == 350.0 {
            let mut height_to_render = rect.h - 300;
            while height_to_render > 0 {
                elements.push((
                    (rect.x - 200, rect.y + rect.h - 150 - height_to_render),
                    Rect::new(0, 300, 200, i32::min(100, height_to_render)),
                ));
                elements.push((
                    (rect.x + rect.w, rect.y + rect.h - 150 - height_to_render),
                    Rect::new(500, 300, 200, i32::min(100, height_to_render)),
                ));
                height_to_render -= 100;
            }
        }

        if shadow_edge_width == 350.0 {
            let mut width_to_render = rect.w - 300;
            while width_to_render > 0 {
                elements.push((
                    (rect.x + rect.w - 150 - width_to_render, rect.y - 200),
                    Rect::new(300, 0, i32::min(100, width_to_render), 200),
                ));
                elements.push((
                    (rect.x + rect.w - 150 - width_to_render, rect.y + rect.h),
                    Rect::new(300, 500, i32::min(100, width_to_render), 200),
                ));
                width_to_render -= 100;
            }
        }

        for (pos, src_rect) in elements {
            self.shadow_texture.render(
                &graphics.renderer,
                1.0,
                pos,
                Some(src_rect),
                false,
                Some(shadow_color),
            );
        }
    }
}
