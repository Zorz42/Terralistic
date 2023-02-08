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
    pub(super) fn translate(&mut self, x: f32, y: f32) {
        let mut transformation = Self::new();
        transformation.matrix = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, 1.0];
        *self = self.clone() * transformation;
    }

    /// Scales the object.
    pub(super) fn stretch(&mut self, x: f32, y: f32) {
        let mut transformation = Self::new();
        transformation.matrix = [x, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 1.0];
        *self = self.clone() * transformation;
    }
}

impl core::ops::Mul for Transformation {
    type Output = Self;

    /// Matrix multiplication
    fn mul(self, other: Self) -> Self {
        // in case you are wondering why I didn't use a loop here:
        // its a lot faster to do it this way
        // loop based approach could according to compiler panic at runtime
        // it was written by ai because I am too lazy to do it myself
        let a11 = self.matrix[0];
        let a21 = self.matrix[1];
        let a31 = self.matrix[2];
        let a12 = self.matrix[3];
        let a22 = self.matrix[4];
        let a32 = self.matrix[5];
        let a13 = self.matrix[6];
        let a23 = self.matrix[7];
        let a33 = self.matrix[8];

        let b11 = other.matrix[0];
        let b21 = other.matrix[1];
        let b31 = other.matrix[2];
        let b12 = other.matrix[3];
        let b22 = other.matrix[4];
        let b32 = other.matrix[5];
        let b13 = other.matrix[6];
        let b23 = other.matrix[7];
        let b33 = other.matrix[8];

        let mut new_transformation = Self::new();
        new_transformation.matrix[0] = a11 * b11 + a12 * b21 + a13 * b31;
        new_transformation.matrix[1] = a21 * b11 + a22 * b21 + a23 * b31;
        new_transformation.matrix[2] = a31 * b11 + a32 * b21 + a33 * b31;
        new_transformation.matrix[3] = a11 * b12 + a12 * b22 + a13 * b32;
        new_transformation.matrix[4] = a21 * b12 + a22 * b22 + a23 * b32;
        new_transformation.matrix[5] = a31 * b12 + a32 * b22 + a33 * b32;
        new_transformation.matrix[6] = a11 * b13 + a12 * b23 + a13 * b33;
        new_transformation.matrix[7] = a21 * b13 + a22 * b23 + a23 * b33;
        new_transformation.matrix[8] = a31 * b13 + a32 * b23 + a33 * b33;

        new_transformation
    }
}
