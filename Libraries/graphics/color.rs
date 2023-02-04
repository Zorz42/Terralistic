#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub const fn set_r(mut self, r: u8) -> Self {
        self.r = r;
        self
    }

    pub const fn set_g(mut self, g: u8) -> Self {
        self.g = g;
        self
    }

    pub const fn set_b(mut self, b: u8) -> Self {
        self.b = b;
        self
    }

    pub const fn set_a(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}
