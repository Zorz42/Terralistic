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
    hovered_slot: Option<usize>,
    hovered_slot_rect: gfx::RenderRect,
}

const INVENTORY_SLOT_SIZE: f32 = 50.0;
const INVENTORY_SPACING: f32 = 10.0;

fn render_inventory_slot(
    graphics: &GraphicsContext,
    items: &ClientItems,
    pos: (f32, f32),
    item: &Option<ItemStack>,
) -> bool {
    let rect = gfx::Rect::new(
        FloatPos(pos.0, pos.1),
        FloatSize(INVENTORY_SLOT_SIZE, INVENTORY_SLOT_SIZE),
    );
    let hovered = rect.contains(graphics.renderer.get_mouse_pos());
    rect.render(
        graphics,
        if hovered {
            gfx::Color::new(100, 100, 100, 255)
        } else {
            gfx::LIGHT_GREY
        },
    );

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

            if item.count > 1 {
                let text_scale = 1.0;
                let text = format!("{}", item.count);
                let text_size = graphics.font.get_text_size_scaled(&text, text_scale);
                let text_pos = FloatPos(
                    rect.pos.0 + rect.size.0 - text_size.0 - 2.0,
                    rect.pos.1 + rect.size.1 - text_size.1 - 2.0,
                );
                graphics
                    .font
                    .render_text(graphics, &text, text_pos, text_scale);
            }
        }
    }

    hovered
}

impl ClientInventory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            back_rect: gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
            inventory: Inventory::new(20),
            open_progress: 0.0,
            hovered_slot: None,
            hovered_slot_rect: gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
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

        self.hovered_slot_rect.size = FloatSize(
            INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING,
            INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING,
        );
        self.hovered_slot_rect.fill_color = gfx::LIGHT_GREY.set_a(gfx::TRANSPARENCY);
        self.hovered_slot_rect.smooth_factor = 40.0;
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext, items: &ClientItems) {
        let open_target = if self.is_open { 1.0 } else { 0.0 };
        self.open_progress += (open_target - self.open_progress) / 10.0;

        if !self.is_open {
            if self.inventory.selected_slot.is_none() {
                self.select_slot(Some(0));
            }

            if let Some(slot) = self.inventory.selected_slot {
                if slot >= 10 {
                    self.select_slot(Some(slot - 10));
                }
            }
        }

        self.back_rect.size.1 = if self.is_open {
            3.0 * INVENTORY_SPACING + 2.0 * INVENTORY_SLOT_SIZE
        } else {
            2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE
        };
        self.back_rect.render(graphics, None);

        if let Some(slot) = self.inventory.selected_slot {
            let x = slot % 10;
            let y = slot / 10;

            let back_rect = *self
                .back_rect
                .get_container(graphics, None)
                .get_absolute_rect();
            self.hovered_slot_rect.pos = FloatPos(
                back_rect.pos.0 + x as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING),
                back_rect.pos.1 + y as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING),
            );
            self.hovered_slot_rect.render(graphics, None);
        }

        let rect = *self
            .back_rect
            .get_container(graphics, None)
            .get_absolute_rect();
        self.hovered_slot = None;
        for (i, item) in self.inventory.reverse_iter().enumerate() {
            let i = 19 - i;
            if i < 10 {
                let result = render_inventory_slot(
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

                if result {
                    self.hovered_slot = Some(i);
                }
            }

            if i >= 10 && self.is_open {
                let animation_spacing = 1.0;

                let pos_y = self.open_progress
                    * (9.0 * animation_spacing + INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                    - (i - 10) as f32 * animation_spacing;

                let result = render_inventory_slot(
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

                if result {
                    self.hovered_slot = Some(i);
                }
            }
        }
    }

    fn select_slot(&mut self, slot: Option<usize>) {
        self.inventory.selected_slot = slot;
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(gfx::Event::KeyPress { 0: key, .. }) = event.downcast::<gfx::Event>() {
            match *key {
                gfx::Key::Num1 => self.select_slot(Some(0)),
                gfx::Key::Num2 => self.select_slot(Some(1)),
                gfx::Key::Num3 => self.select_slot(Some(2)),
                gfx::Key::Num4 => self.select_slot(Some(3)),
                gfx::Key::Num5 => self.select_slot(Some(4)),
                gfx::Key::Num6 => self.select_slot(Some(5)),
                gfx::Key::Num7 => self.select_slot(Some(6)),
                gfx::Key::Num8 => self.select_slot(Some(7)),
                gfx::Key::Num9 => self.select_slot(Some(8)),
                gfx::Key::Num0 => self.select_slot(Some(9)),
                gfx::Key::MouseLeft => {
                    if self.inventory.selected_slot == self.hovered_slot {
                        self.select_slot(None);
                    } else {
                        self.select_slot(self.hovered_slot);
                    }
                }
                gfx::Key::E => {
                    self.is_open = !self.is_open;
                }
                _ => {}
            }
        }

        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<InventoryPacket>() {
                self.inventory = packet.inventory;
            }
        }
    }
}
