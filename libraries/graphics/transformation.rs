/**
Transformation is a 3x3 matrix used to move and scale drawn objects.
 */
#[derive(Clone)]
pub(crate) struct Transformation {
    pub matrix: [f32; 9],
}

impl Transformation {
    /**
    A new Transformation always contains identity matrix.
     */
    pub fn new() -> Transformation {
        Transformation {
            matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    /**
    Moves the object around.
     */
    pub(crate) fn translate(&mut self, x: f32, y: f32) {
        let mut transformation = Self::new();
        transformation.matrix = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, 1.0];
        *self = self.clone() * transformation;
    }

    /**
    Scales the object.
     */
    pub(crate) fn stretch(&mut self, x: f32, y: f32) {
        let mut transformation = Self::new();
        transformation.matrix = [x, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 1.0];
        *self = self.clone() * transformation;
    }
}

impl std::ops::Mul for Transformation {
    type Output = Transformation;

    /**
    Matrix multiplication
     */
    fn mul(self, other: Self) -> Self {
        let mut new_transformation = Self::new();
        for x in 0..3 {
            for y in 0..3 {
                let mut sum = 0.0;
                for i in 0..3 {
                    sum += self.matrix[3 * i + y] * other.matrix[3 * x + i];
                }
                new_transformation.matrix[3 * x + y] = sum;
            }
        }
        new_transformation
    }
}
