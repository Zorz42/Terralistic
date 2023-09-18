use crate::libraries::graphics as gfx;

/// Transformation is a 3x3 matrix used to move and scale drawn objects.
#[derive(Clone)]
pub struct Transformation {
    pub matrix: [f32; 9],
}

impl Transformation {
    /// A new Transformation always contains identity matrix.
    pub const fn new() -> Self {
        Self {
            matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Moves the object around.
    pub(super) fn translate(&mut self, pos: gfx::FloatPos) {
        self.matrix[6] += self.matrix[0] * pos.0 + self.matrix[3] * pos.1;
        self.matrix[7] += self.matrix[1] * pos.0 + self.matrix[4] * pos.1;
        self.matrix[8] += self.matrix[2] * pos.0 + self.matrix[5] * pos.1;
    }

    /// Scales the object.
    pub(super) fn stretch(&mut self, scale: (f32, f32)) {
        self.matrix[0] *= scale.0;
        self.matrix[1] *= scale.0;
        self.matrix[2] *= scale.0;
        self.matrix[3] *= scale.1;
        self.matrix[4] *= scale.1;
        self.matrix[5] *= scale.1;
    }
}
