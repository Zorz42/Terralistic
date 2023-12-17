use crate::libraries::graphics as gfx;

/// `ShadowContext` is a struct that contains the information needed to draw a shadow.
pub struct ShadowContext {
    pub shadow_texture: gfx::Texture,
}

impl ShadowContext {
    /// Sets the pixel at the given position to the given color.
    /// The color is blended with the previous color using the alpha channel.
    /// The alpha channel is calculated using a gaussian curve.
    fn set_gaussian_pixel(h: i32, pixel: &mut gfx::Color) {
        let alpha = std::f32::consts::E.powf(-((h * h) as f32 / 2000.0));
        let prev_alpha = pixel.a as f32 / 255.0;
        *pixel = gfx::Color::new(0, 0, 0, (alpha * prev_alpha * 255.0) as u8);
    }

    /// Creates a new `ShadowContext`.
    pub fn new() -> Self {
        let mut surface = gfx::Surface::new(gfx::IntSize(700, 700));
        // fill surface with black color
        for (_, pixel) in surface.iter_mut() {
            *pixel = gfx::Color::new(0, 0, 0, 255);
        }

        // draw a shadow to the surface with center rectangle being transparent with
        // size 300, 300 and offset 200, 200. The blur around the rectangle is a gaussian
        // curve.

        for (pos, pixel) in surface.iter_mut() {
            if pos.1 < 200 {
                Self::set_gaussian_pixel(200 - pos.1, pixel);
            }

            if pos.0 < 200 {
                Self::set_gaussian_pixel(200 - pos.0, pixel);
            }

            if pos.1 > 500 {
                Self::set_gaussian_pixel(pos.1 - 500, pixel);
            }

            if pos.0 > 500 {
                Self::set_gaussian_pixel(pos.0 - 500, pixel);
            }
        }

        let shadow_texture = gfx::Texture::load_from_surface(&surface);

        Self { shadow_texture }
    }

    /// Renders the shadow.
    #[allow(clippy::too_many_lines)]
    pub fn render(&self, graphics: &gfx::GraphicsContext, rect: &gfx::Rect, shadow_intensity: f32) {
        let shadow_color = gfx::Color::new(0, 0, 0, (80.0 * shadow_intensity) as u8);

        let shadow_edge_width = f32::min(200.0 + rect.size.0 / 2.0, 350.0);
        let shadow_edge_height = f32::min(200.0 + rect.size.1 / 2.0, 350.0);

        let mut elements = vec![
            (
                rect.pos - gfx::FloatPos(200.0, 200.0),
                gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(shadow_edge_width, 200.0)),
            ),
            (
                rect.pos - gfx::FloatPos(200.0, 0.0),
                gfx::Rect::new(gfx::FloatPos(0.0, 200.0), gfx::FloatSize(200.0, shadow_edge_height - 200.0)),
            ),
            (
                rect.pos + gfx::FloatPos(rect.size.0 - shadow_edge_width + 200.0, -200.0),
                gfx::Rect::new(gfx::FloatPos(700.0 - shadow_edge_width, 0.0), gfx::FloatSize(shadow_edge_width, 200.0)),
            ),
            (
                rect.pos + gfx::FloatPos(rect.size.0, 0.0),
                gfx::Rect::new(gfx::FloatPos(500.0, 200.0), gfx::FloatSize(200.0, shadow_edge_height - 200.0)),
            ),
            (
                rect.pos + gfx::FloatPos(-200.0, rect.size.1 - shadow_edge_height + 200.0),
                gfx::Rect::new(gfx::FloatPos(0.0, 700.0 - shadow_edge_height), gfx::FloatSize(200.0, shadow_edge_height - 200.0)),
            ),
            (
                rect.pos + gfx::FloatPos(-200.0, rect.size.1),
                gfx::Rect::new(gfx::FloatPos(0.0, 500.0), gfx::FloatSize(shadow_edge_width, 200.0)),
            ),
            (
                rect.pos + gfx::FloatPos(rect.size.0, rect.size.1 - shadow_edge_height + 200.0),
                gfx::Rect::new(gfx::FloatPos(500.0, 700.0 - shadow_edge_height), gfx::FloatSize(200.0, shadow_edge_height - 200.0)),
            ),
            (
                rect.pos + gfx::FloatPos(rect.size.0 - shadow_edge_width + 200.0, rect.size.1),
                gfx::Rect::new(gfx::FloatPos(700.0 - shadow_edge_width, 500.0), gfx::FloatSize(shadow_edge_width, 200.0)),
            ),
        ];

        if (shadow_edge_height - 350.0).abs() < f32::EPSILON {
            let mut height_to_render = rect.size.1 - 300.0;
            while height_to_render > 0.0 {
                elements.push((
                    rect.pos + gfx::FloatPos(-200.0, rect.size.1 - 150.0 - height_to_render),
                    gfx::Rect::new(gfx::FloatPos(0.0, 300.0), gfx::FloatSize(200.0, f32::min(100.0, height_to_render))),
                ));
                elements.push((
                    rect.pos + gfx::FloatPos(rect.size.0, rect.size.1 - 150.0 - height_to_render),
                    gfx::Rect::new(gfx::FloatPos(500.0, 300.0), gfx::FloatSize(200.0, f32::min(100.0, height_to_render))),
                ));
                height_to_render -= 100.0;
            }
        }

        if (shadow_edge_width - 350.0).abs() < f32::EPSILON {
            let mut width_to_render = rect.size.0 - 300.0;
            while width_to_render > 0.0 {
                elements.push((
                    rect.pos + gfx::FloatPos(rect.size.0 - 150.0 - width_to_render, -200.0),
                    gfx::Rect::new(gfx::FloatPos(300.0, 0.0), gfx::FloatSize(f32::min(100.0, width_to_render), 200.0)),
                ));
                elements.push((
                    rect.pos + gfx::FloatPos(rect.size.0 - 150.0 - width_to_render, rect.size.1),
                    gfx::Rect::new(gfx::FloatPos(300.0, 500.0), gfx::FloatSize(f32::min(100.0, width_to_render), 200.0)),
                ));
                width_to_render -= 100.0;
            }
        }

        for (pos, src_rect) in elements {
            self.shadow_texture.render(graphics, 1.0, pos, Some(src_rect), false, Some(shadow_color));
        }
    }
}
