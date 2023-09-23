use crate::client::game::items::ClientItems;
use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::shared::inventory::{
    Inventory, InventoryPacket, InventorySelectPacket, InventorySwapPacket,
};
use crate::shared::items::ItemStack;
use crate::shared::packet::Packet;
use anyhow::{anyhow, Result};

pub struct ClientInventory {
    is_open: bool,
    back_rect: gfx::RenderRect,
    inventory: Inventory,
    open_progress: f32,
    hovered_slot: Option<usize>,
    hovered_slot_rect: gfx::RenderRect,
    lower_slots_pos: [f32; 10],
}

const INVENTORY_SLOT_SIZE: f32 = 50.0;
const INVENTORY_SPACING: f32 = 10.0;

fn render_item_stack(
    graphics: &gfx::GraphicsContext,
    items: &ClientItems,
    pos: (f32, f32),
    item: &Option<ItemStack>,
) {
    if let Some(item) = item {
        let src_rect = items.get_atlas().get_rect(&item.item);
        if let Some(src_rect) = src_rect {
            let mut src_rect = *src_rect;
            src_rect.size.0 /= 2.0;

            let scale = 3.0;
            let texture = items.get_atlas().get_texture();
            let item_pos = gfx::FloatPos(
                pos.0 + INVENTORY_SLOT_SIZE / 2.0 - src_rect.size.0 / 2.0 * scale,
                pos.1 + INVENTORY_SLOT_SIZE / 2.0 - src_rect.size.1 / 2.0 * scale,
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
                let text_pos = gfx::FloatPos(
                    pos.0 + INVENTORY_SLOT_SIZE - text_size.0 - 2.0,
                    pos.1 + INVENTORY_SLOT_SIZE - text_size.1 - 2.0,
                );
                graphics
                    .font
                    .render_text(graphics, &text, text_pos, text_scale);
            }
        }
    }
}

fn render_inventory_slot(
    graphics: &gfx::GraphicsContext,
    items: &ClientItems,
    pos: (f32, f32),
    item: &Option<ItemStack>,
) -> bool {
    let rect = gfx::Rect::new(
        gfx::FloatPos(pos.0, pos.1),
        gfx::FloatSize(INVENTORY_SLOT_SIZE, INVENTORY_SLOT_SIZE),
    );
    let hovered = rect.contains(graphics.renderer.get_mouse_pos());
    rect.render(graphics, if hovered { gfx::GREY } else { gfx::DARK_GREY });

    render_item_stack(graphics, items, pos, item);

    hovered
}

impl ClientInventory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            inventory: Inventory::new(20),
            open_progress: 0.0,
            hovered_slot: None,
            hovered_slot_rect: gfx::RenderRect::new(
                gfx::FloatPos(0.0, 0.0),
                gfx::FloatSize(0.0, 0.0),
            ),
            lower_slots_pos: [0.0; 10],
        }
    }

    pub fn init(&mut self) {
        self.back_rect.orientation = gfx::TOP;
        self.back_rect.pos = gfx::FloatPos(0.0, INVENTORY_SPACING);
        self.back_rect.size = gfx::FloatSize(
            10.0 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING) + INVENTORY_SPACING,
            2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE,
        );
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.blur_radius = gfx::BLUR / 2;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY / 2;

        self.hovered_slot_rect.size = gfx::FloatSize(
            INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING,
            INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING,
        );
        self.hovered_slot_rect.fill_color = gfx::BLACK.set_a(gfx::TRANSPARENCY);
        self.hovered_slot_rect.smooth_factor = 40.0;
    }

    #[allow(clippy::too_many_lines)]
    pub fn render(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        items: &ClientItems,
        networking: &mut ClientNetworking,
    ) -> Result<()> {
        let open_target = if self.is_open { 1.0 } else { 0.0 };
        self.open_progress += (open_target - self.open_progress) / 5.0;

        if !self.is_open {
            if self.inventory.selected_slot.is_none() {
                self.select_slot(Some(0), networking)?;
            }

            if let Some(slot) = self.inventory.selected_slot {
                if slot >= 10 {
                    self.select_slot(Some(slot - 10), networking)?;
                }
            }
        }

        self.back_rect.size.1 = self.open_progress
            * (3.0 * INVENTORY_SPACING + 2.0 * INVENTORY_SLOT_SIZE)
            + (1.0 - self.open_progress) * (2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE);
        self.back_rect.render(graphics, None);

        if let Some(slot) = self.inventory.selected_slot {
            let x = slot % 10;
            let y = slot / 10;

            let back_rect = *self
                .back_rect
                .get_container(graphics, None)
                .get_absolute_rect();
            self.hovered_slot_rect.pos = gfx::FloatPos(
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
                let item = if self.is_open && self.inventory.selected_slot == Some(i) {
                    &None
                } else {
                    item
                };

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

            if i >= 10 {
                let pos_y = self
                    .lower_slots_pos
                    .get_mut(i - 10)
                    .ok_or_else(|| anyhow!("indexing error"))?;

                let target_y = if self.is_open && self.open_progress > 0.07 * (i - 9) as f32 {
                    INVENTORY_SLOT_SIZE + INVENTORY_SPACING
                } else {
                    0.0
                };

                *pos_y += (target_y - *pos_y) / 5.0;

                let item = if self.is_open && self.inventory.selected_slot == Some(i) {
                    &None
                } else {
                    item
                };

                if *pos_y > 0.0 {
                    let result = render_inventory_slot(
                        graphics,
                        items,
                        (
                            rect.pos.0
                                + (i - 10) as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                                + INVENTORY_SPACING,
                            rect.pos.1 + INVENTORY_SPACING + *pos_y,
                        ),
                        item,
                    );

                    if result {
                        self.hovered_slot = Some(i);
                    }
                }
            }
        }

        if self.is_open {
            render_item_stack(
                graphics,
                items,
                (
                    graphics.renderer.get_mouse_pos().0,
                    graphics.renderer.get_mouse_pos().1,
                ),
                &self.inventory.get_selected_item(),
            );
        }

        Ok(())
    }

    fn select_slot(
        &mut self,
        slot: Option<usize>,
        networking: &mut ClientNetworking,
    ) -> Result<()> {
        self.inventory.selected_slot = slot;
        let packet = InventorySelectPacket { slot };
        networking.send_packet(Packet::new(packet)?)
    }

    fn swap_slot_with_selected_slot(
        &mut self,
        slot: usize,
        networking: &mut ClientNetworking,
    ) -> Result<()> {
        self.inventory.swap_with_selected_item(slot)?;
        let packet = InventorySwapPacket { slot };
        networking.send_packet(Packet::new(packet)?)
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ClientNetworking) -> Result<()> {
        if let Some(gfx::Event::KeyPress { 0: key, .. }) = event.downcast::<gfx::Event>() {
            match *key {
                gfx::Key::Num1 => self.select_slot(Some(0), networking)?,
                gfx::Key::Num2 => self.select_slot(Some(1), networking)?,
                gfx::Key::Num3 => self.select_slot(Some(2), networking)?,
                gfx::Key::Num4 => self.select_slot(Some(3), networking)?,
                gfx::Key::Num5 => self.select_slot(Some(4), networking)?,
                gfx::Key::Num6 => self.select_slot(Some(5), networking)?,
                gfx::Key::Num7 => self.select_slot(Some(6), networking)?,
                gfx::Key::Num8 => self.select_slot(Some(7), networking)?,
                gfx::Key::Num9 => self.select_slot(Some(8), networking)?,
                gfx::Key::Num0 => self.select_slot(Some(9), networking)?,
                gfx::Key::MouseLeft => {
                    if self.inventory.selected_slot == self.hovered_slot {
                        self.select_slot(None, networking)?;
                    } else if let Some(hovered_slot) = self.hovered_slot {
                        if self.inventory.get_selected_item().is_some() {
                            self.swap_slot_with_selected_slot(hovered_slot, networking)?;
                            self.select_slot(None, networking)?;
                        } else {
                            self.select_slot(self.hovered_slot, networking)?;
                        }
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
                self.inventory.transfer_items_from(packet.inventory);
            }
        }

        Ok(())
    }
}
