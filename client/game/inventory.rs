use crate::libraries::graphics::GraphicsContext;

pub struct ClientInventory {
    is_open: bool,
}

impl ClientInventory {
    #[must_use]
    pub const fn new() -> Self {
        Self { is_open: false }
    }

    pub fn init(&mut self) {}

    pub fn render(&mut self, graphics: &mut GraphicsContext) {}
}
