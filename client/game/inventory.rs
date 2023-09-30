use anyhow::{anyhow, Result};

use crate::client::game::items::ClientItems;
use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::shared::inventory::{
    Inventory, InventoryCraftPacket, InventoryPacket, InventorySelectPacket, InventorySwapPacket,
};
use crate::shared::items::{ItemStack, RecipeId};
use crate::shared::packet::Packet;

pub struct ClientInventory {
    is_open: bool,
    back_rect: gfx::RenderRect,
    inventory: Inventory,
    open_progress: f32,
    hovered_slot: Option<usize>,
    hovered_slot_rect: gfx::RenderRect,
    lower_slots_pos: [f32; 10],
    craftable_recipes: Vec<RecipeId>,
    crafting_back_rect: gfx::RenderRect,
    hover_back_rect: gfx::RenderRect,
    hovered_recipe: Option<RecipeId>,
}

const INVENTORY_SLOT_SIZE: f32 = 50.0;
const INVENTORY_SPACING: f32 = 10.0;

fn render_item_stack(
    graphics: &gfx::GraphicsContext,
    items: &ClientItems,
    pos: gfx::FloatPos,
    item: Option<&ItemStack>,
) {
    let pos = gfx::FloatPos(pos.0.round(), pos.1.round());

    if let Some(item) = item {
        let src_rect = items.get_atlas().get_rect(&item.item);
        if let Some(src_rect) = src_rect {
            let mut src_rect = *src_rect;
            src_rect.size.0 /= 2.0;

            let scale = 3.0;
            let texture = items.get_atlas().get_texture();
            let item_pos = pos
                + gfx::FloatPos(
                    INVENTORY_SLOT_SIZE / 2.0 - src_rect.size.0 / 2.0 * scale,
                    INVENTORY_SLOT_SIZE / 2.0 - src_rect.size.1 / 2.0 * scale,
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
                let text_size = graphics.font.get_text_size_scaled(&text, text_scale, None);
                let text_pos = pos
                    + gfx::FloatPos(INVENTORY_SLOT_SIZE - 2.0, INVENTORY_SLOT_SIZE - 2.0)
                    - text_size;
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
    pos: gfx::FloatPos,
    item: Option<&ItemStack>,
) -> bool {
    let rect = gfx::Rect::new(
        pos,
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
            craftable_recipes: Vec::new(),
            crafting_back_rect: gfx::RenderRect::new(
                gfx::FloatPos(0.0, 0.0),
                gfx::FloatSize(0.0, 0.0),
            ),
            hover_back_rect: gfx::RenderRect::new(
                gfx::FloatPos(0.0, 0.0),
                gfx::FloatSize(0.0, INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING),
            ),
            hovered_recipe: None,
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

        self.crafting_back_rect.pos.1 = INVENTORY_SPACING;
        self.crafting_back_rect.size.0 = INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING;
        self.crafting_back_rect.fill_color = gfx::BLACK;
        self.crafting_back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.crafting_back_rect.blur_radius = gfx::BLUR / 2;
        self.crafting_back_rect.shadow_intensity = gfx::SHADOW_INTENSITY / 2;

        self.hover_back_rect.fill_color = gfx::BLACK;
        self.hover_back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.hover_back_rect.blur_radius = gfx::BLUR / 2;
        self.hover_back_rect.shadow_intensity = gfx::SHADOW_INTENSITY / 2;
    }

    fn update_craftable_recipes(&mut self, items: &ClientItems) {
        self.craftable_recipes.clear();
        let items = items.get_items();
        let recipes = items.get_recipes();
        for recipe in recipes {
            if self.inventory.can_craft(recipe) {
                self.craftable_recipes.push(recipe.get_id());
            }
        }
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
                    None
                } else {
                    item.as_ref()
                };

                let result = render_inventory_slot(
                    graphics,
                    items,
                    rect.pos
                        + gfx::FloatPos(
                            i as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                                + INVENTORY_SPACING,
                            INVENTORY_SPACING,
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
                    None
                } else {
                    item.as_ref()
                };

                if *pos_y > 0.0 {
                    let result = render_inventory_slot(
                        graphics,
                        items,
                        rect.pos
                            + gfx::FloatPos(
                                (i - 10) as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                                    + INVENTORY_SPACING,
                                INVENTORY_SPACING + *pos_y,
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
                graphics.renderer.get_mouse_pos(),
                self.inventory.get_selected_item().as_ref(),
            );
        }

        self.hovered_recipe = None;
        if self.open_progress > 0.0 {
            self.crafting_back_rect.pos.0 = INVENTORY_SPACING * self.open_progress
                + (-self.crafting_back_rect.size.0 - INVENTORY_SPACING)
                    * (1.0 - self.open_progress);

            self.crafting_back_rect.size.1 = INVENTORY_SPACING
                * (self.craftable_recipes.len() as f32 + 1.0)
                + INVENTORY_SLOT_SIZE * self.craftable_recipes.len() as f32;

            if !self.craftable_recipes.is_empty() {
                self.crafting_back_rect.render(graphics, None);
            }

            let x = self.crafting_back_rect.pos.0 + INVENTORY_SPACING;
            let mut y = self.crafting_back_rect.pos.1 + INVENTORY_SPACING;

            for recipe_id in &self.craftable_recipes {
                let items2 = items.get_items();
                let recipe = items2.get_recipe(*recipe_id)?;
                let item = recipe.result.clone();
                let hovered =
                    render_inventory_slot(graphics, items, gfx::FloatPos(x, y), Some(&item));

                if hovered {
                    self.hovered_recipe = Some(*recipe_id);

                    let num_recipes = recipe.ingredients.len() as i32;

                    self.hover_back_rect.pos = graphics.renderer.get_mouse_pos();
                    self.hover_back_rect.size.0 = num_recipes as f32
                        * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING)
                        + INVENTORY_SPACING;

                    self.hover_back_rect.render(graphics, None);

                    let mut x = graphics.renderer.get_mouse_pos().0 + INVENTORY_SPACING;
                    let y = graphics.renderer.get_mouse_pos().1 + INVENTORY_SPACING;

                    for (item, count) in &recipe.ingredients {
                        render_inventory_slot(
                            graphics,
                            items,
                            gfx::FloatPos(x, y),
                            Some(&ItemStack::new(*item, *count)),
                        );
                        x += INVENTORY_SLOT_SIZE + INVENTORY_SPACING;
                    }
                }

                y += INVENTORY_SPACING + INVENTORY_SLOT_SIZE;
            }
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

    pub fn on_event(
        &mut self,
        event: &Event,
        networking: &mut ClientNetworking,
        items: &mut ClientItems,
    ) -> Result<()> {
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

                    if let Some(recipe) = self.hovered_recipe {
                        networking.send_packet(Packet::new(InventoryCraftPacket { recipe })?)?;
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
                self.update_craftable_recipes(items);
            }
        }

        Ok(())
    }
}
