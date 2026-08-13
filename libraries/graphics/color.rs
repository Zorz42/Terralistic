use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    pub const fn set_r(mut self, r: u8) -> Self {
        self.r = r;
        self
    }

    #[must_use]
    pub const fn set_g(mut self, g: u8) -> Self {
        self.g = g;
        self
    }

    #[must_use]
    pub const fn set_b(mut self, b: u8) -> Self {
        self.b = b;
        self
    }

    #[must_use]
    pub const fn set_a(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}

/// Mixes two colours channel by channel, `t` of the way from `a` to `b`.
#[must_use]
pub fn interpolate_colors(a: Color, b: Color, t: f32) -> Color {
    let mix = |from: u8, to: u8| (from as f32 * (1.0 - t) + to as f32 * t) as u8;
    Color::new(mix(a.r, b.r), mix(a.g, b.g), mix(a.b, b.b), mix(a.a, b.a))
}
