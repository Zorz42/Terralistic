use crate::color;
use crate::renderer;

/*
This is a rectangle shape not really
meant to be rendered.
*/
pub struct RectShape {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl RectShape {
    /*
    Renders the rectangle on the screen.
    */
    pub fn render(&self, renderer: &renderer::Renderer, color: color::Color) {
        if color.a == 0 {
            return;
        }

        let mut transform = renderer.normalization_transform.clone();
        transform.translate(self.x as f32, self.y as f32);
        transform.stretch(self.w as f32, self.h as f32);

        unsafe {
            gl::UniformMatrix3fv(renderer.uniforms.transform_matrix, 1, gl::FALSE, &transform.matrix[0]);
            gl::Uniform4f(renderer.uniforms.global_color, color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);
            gl::Uniform1i(renderer.uniforms.has_texture, 0);
        }

        renderer.rect_vertex_buffer.draw(false);
    }
}