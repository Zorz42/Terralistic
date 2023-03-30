use crate::client::game::items::ClientItems;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::shared::inventory::Inventory;
use crate::shared::items::ItemStack;
use crate::shared::packet::Packet;
use crate::shared::players::InventoryPacket;

pub struct ClientInventory {
    is_open: bool,
    back_rect: gfx::RenderRect,
    inventory: Inventory,
    open_progress: f32,
}

const INVENTORY_SLOT_SIZE: f32 = 50.0;
const INVENTORY_SPACING: f32 = 10.0;

fn render_inventory_slot(
    graphics: &GraphicsContext,
    items: &ClientItems,
    pos: (f32, f32),
    item: &Option<ItemStack>,
) {
    let rect = gfx::Rect::new(
        FloatPos(pos.0, pos.1),
        FloatSize(INVENTORY_SLOT_SIZE, INVENTORY_SLOT_SIZE),
    );
    rect.render(graphics, gfx::GREY);

    if let Some(item) = item {
        let src_rect = items.get_atlas().get_rect(&item.item);
        if let Some(src_rect) = src_rect {
            let mut src_rect = *src_rect;
            src_rect.pos.0 += 1.0;
            src_rect.pos.1 += 2.0;
            src_rect.size = FloatSize(8.0, 8.0);

            let scale = 4.0;
            let texture = items.get_atlas().get_texture();
            let item_pos = FloatPos(
                rect.pos.0 + rect.size.0 / 2.0 - src_rect.size.0 / 2.0 * scale,
                rect.pos.1 + rect.size.1 / 2.0 - src_rect.size.1 / 2.0 * scale,
            );
            texture.render(
                &graphics.renderer,
                scale,
                item_pos,
                Some(src_rect),
                false,
                None,
            );
        }
    }
}

impl ClientInventory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            back_rect: gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
            inventory: Inventory::new(20),
            open_progress: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.back_rect.orientation = gfx::TOP;
        self.back_rect.pos = FloatPos(0.0, INVENTORY_SPACING);
        self.back_rect.size = FloatSize(
            10.0 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING) + INVENTORY_SPACING,
            2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE,
        );
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.blur_radius = gfx::BLUR / 2;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY / 2;
        self.back_rect.smooth_factor = 20.0;
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext, items: &ClientItems) {
        let open_target = if self.is_open { 1.0 } else { 0.0 };
        self.open_progress += (open_target - self.open_progress) / 10.0;

        self.back_rect.size.1 = if self.is_open {
            3.0 * INVENTORY_SPACING + 2.0 * INVENTORY_SLOT_SIZE
        } else {
            2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE
        };
        self.back_rect.render(graphics, None);

        let rect = *self
            .back_rect
            .get_container(graphics, None)
            .get_absolute_rect();
        for (i, item) in self.inventory.reverse_iter().enumerate() {
            let i = 19 - i;
            if i < 10 {
                render_inventory_slot(
                    graphics,
                    items,
                    (
                        rect.pos.0
                            + i as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                            + INVENTORY_SPACING,
                        rect.pos.1 + INVENTORY_SPACING,
                    ),
                    item,
                );
            }

            if i >= 10 && self.is_open {
                let animation_spacing = 1.0;

                let pos_y = self.open_progress
                    * (9.0 * animation_spacing + INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                    - (i - 10) as f32 * animation_spacing;

                render_inventory_slot(
                    graphics,
                    items,
                    (
                        rect.pos.0
                            + (i - 10) as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                            + INVENTORY_SPACING,
                        rect.pos.1
                            + INVENTORY_SPACING
                            + pos_y.clamp(0.0, INVENTORY_SPACING + INVENTORY_SLOT_SIZE),
                    ),
                    item,
                );
            }
        }
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(gfx::Event::KeyPress { 0: key, .. }) = event.downcast::<gfx::Event>() {
            if *key == gfx::Key::E {
                self.is_open = !self.is_open;
            }
        }

        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<InventoryPacket>() {
                self.inventory = packet.inventory;
            }
        }
    }
}
