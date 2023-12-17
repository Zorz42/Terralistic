use crate::client::game::block_selector::BlockRightClickEvent;
use anyhow::{anyhow, Result};

use crate::client::game::items::ClientItems;
use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::shared::blocks::Blocks;
use crate::shared::inventory::{Inventory, InventoryCraftPacket, InventoryPacket, InventorySelectPacket, InventorySwapPacket, Slot};
use crate::shared::items::{ItemStack, RecipeId};
use crate::shared::packet::Packet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OpenState {
    Closed,
    Open,
    OpenedBlock { x: i32, y: i32 },
}

enum HoveredSlot {
    None,
    Inventory(usize),
    Block(usize),
    Recipe(RecipeId),
}

pub struct ClientInventory {
    open_state: OpenState,
    back_rect: gfx::RenderRect,
    inventory: Inventory,
    open_progress: f32,
    hovered_slot: HoveredSlot,
    hovered_slot_rect: gfx::RenderRect,
    lower_slots_pos: [f32; 10],
    craftable_recipes: Vec<RecipeId>,
    crafting_back_rect: gfx::RenderRect,
    hover_back_rect: gfx::RenderRect,
}

const INVENTORY_SLOT_SIZE: f32 = 50.0;
const INVENTORY_SPACING: f32 = 10.0;

fn render_item_stack(graphics: &gfx::GraphicsContext, items: &ClientItems, pos: gfx::FloatPos, item: Option<&ItemStack>) {
    let pos = gfx::FloatPos(pos.0.round(), pos.1.round());

    if let Some(item) = item {
        let src_rect = items.get_atlas().get_rect(&item.item);
        if let Some(src_rect) = src_rect {
            let mut src_rect = *src_rect;
            src_rect.size.0 /= 2.0;

            let scale = 3.0;
            let texture = items.get_atlas().get_texture();
            let item_pos = pos + gfx::FloatPos(INVENTORY_SLOT_SIZE / 2.0 - src_rect.size.0 / 2.0 * scale, INVENTORY_SLOT_SIZE / 2.0 - src_rect.size.1 / 2.0 * scale);
            texture.render(graphics, scale, item_pos, Some(src_rect), false, None);

            if item.count > 1 {
                let text_scale = 1.0;
                let text = format!("{}", item.count);
                let text_size = graphics.font.get_text_size_scaled(&text, text_scale, None);
                let text_pos = pos + gfx::FloatPos(INVENTORY_SLOT_SIZE - 2.0, INVENTORY_SLOT_SIZE - 2.0) - text_size;
                graphics.font.render_text(graphics, &text, text_pos, text_scale);
            }
        }
    }
}

fn render_inventory_slot(graphics: &gfx::GraphicsContext, items: &ClientItems, pos: gfx::FloatPos, item: Option<&ItemStack>) -> bool {
    let rect = gfx::Rect::new(pos, gfx::FloatSize(INVENTORY_SLOT_SIZE, INVENTORY_SLOT_SIZE));
    let hovered = rect.contains(graphics.get_mouse_pos());
    rect.render(graphics, if hovered { gfx::GREY } else { gfx::DARK_GREY });

    render_item_stack(graphics, items, pos, item);

    hovered
}

impl ClientInventory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            open_state: OpenState::Closed,
            back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            inventory: Inventory::new(20),
            open_progress: 0.0,
            hovered_slot: HoveredSlot::None,
            hovered_slot_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            lower_slots_pos: [0.0; 10],
            craftable_recipes: Vec::new(),
            crafting_back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            hover_back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING)),
        }
    }

    pub fn init(&mut self) {
        self.back_rect.orientation = gfx::TOP;
        self.back_rect.pos = gfx::FloatPos(0.0, INVENTORY_SPACING);
        self.back_rect.size = gfx::FloatSize(10.0 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING) + INVENTORY_SPACING, 2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE);
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.blur_radius = gfx::BLUR / 2;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY / 2;

        self.hovered_slot_rect.size = gfx::FloatSize(INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING, INVENTORY_SLOT_SIZE + 2.0 * INVENTORY_SPACING);
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

    fn render_inventory(&mut self, graphics: &gfx::GraphicsContext, items: &ClientItems) -> Result<()> {
        self.back_rect.size.1 = self.open_progress * (3.0 * INVENTORY_SPACING + 2.0 * INVENTORY_SLOT_SIZE) + (1.0 - self.open_progress) * (2.0 * INVENTORY_SPACING + INVENTORY_SLOT_SIZE);
        self.back_rect.render(graphics, None);

        if let Some(slot) = self.inventory.selected_slot {
            let x = slot % 10;
            let y = slot / 10;

            let back_rect = *self.back_rect.get_container(graphics, None).get_absolute_rect();
            self.hovered_slot_rect.pos = gfx::FloatPos(
                back_rect.pos.0 + x as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING),
                back_rect.pos.1 + y as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING),
            );
            self.hovered_slot_rect.render(graphics, None);
        }

        let rect = *self.back_rect.get_container(graphics, None).get_absolute_rect();
        for (i, item) in self.inventory.reverse_iter().enumerate() {
            let i = 19 - i;
            if i < 10 {
                let item = if self.open_state != OpenState::Closed && self.inventory.selected_slot == Some(i) {
                    None
                } else {
                    item.as_ref()
                };

                let result = render_inventory_slot(
                    graphics,
                    items,
                    rect.pos + gfx::FloatPos(i as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING) + INVENTORY_SPACING, INVENTORY_SPACING),
                    item,
                );

                if result {
                    self.hovered_slot = HoveredSlot::Inventory(i);
                }
            }

            if i >= 10 {
                let pos_y = self.lower_slots_pos.get_mut(i - 10).ok_or_else(|| anyhow!("indexing error"))?;

                let target_y = if self.open_state != OpenState::Closed && self.open_progress > 0.07 * (i - 9) as f32 {
                    INVENTORY_SLOT_SIZE + INVENTORY_SPACING
                } else {
                    0.0
                };

                *pos_y += (target_y - *pos_y) / 5.0;

                let item = if self.open_state != OpenState::Closed && self.inventory.selected_slot == Some(i) {
                    None
                } else {
                    item.as_ref()
                };

                if *pos_y > 0.0 {
                    let result = render_inventory_slot(
                        graphics,
                        items,
                        rect.pos + gfx::FloatPos((i - 10) as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING) + INVENTORY_SPACING, INVENTORY_SPACING + *pos_y),
                        item,
                    );

                    if result {
                        self.hovered_slot = HoveredSlot::Inventory(i);
                    }
                }
            }
        }
        Ok(())
    }

    fn render_crafting(&mut self, graphics: &gfx::GraphicsContext, items: &ClientItems) -> Result<()> {
        if self.open_progress > 0.0 {
            self.crafting_back_rect.pos.0 = INVENTORY_SPACING * self.open_progress + (-self.crafting_back_rect.size.0 - INVENTORY_SPACING) * (1.0 - self.open_progress);

            self.crafting_back_rect.size.1 = INVENTORY_SPACING * (self.craftable_recipes.len() as f32 + 1.0) + INVENTORY_SLOT_SIZE * self.craftable_recipes.len() as f32;

            if !self.craftable_recipes.is_empty() {
                self.crafting_back_rect.render(graphics, None);
            }

            let x = self.crafting_back_rect.pos.0 + INVENTORY_SPACING;
            let mut y = self.crafting_back_rect.pos.1 + INVENTORY_SPACING;

            for recipe_id in &self.craftable_recipes {
                let items2 = items.get_items();
                let recipe = items2.get_recipe(*recipe_id)?;
                let item = recipe.result.clone();
                let hovered = render_inventory_slot(graphics, items, gfx::FloatPos(x, y), Some(&item));

                if hovered {
                    self.hovered_slot = HoveredSlot::Recipe(*recipe_id);
                }

                y += INVENTORY_SPACING + INVENTORY_SLOT_SIZE;
            }
        }

        if let HoveredSlot::Recipe(recipe_id) = self.hovered_slot {
            let items2 = items.get_items();
            let recipe = items2.get_recipe(recipe_id)?;
            let num_recipes = recipe.ingredients.len() as i32;

            self.hover_back_rect.pos = graphics.get_mouse_pos();
            self.hover_back_rect.size.0 = num_recipes as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_SPACING) + INVENTORY_SPACING;

            self.hover_back_rect.render(graphics, None);

            let mut x = graphics.get_mouse_pos().0 + INVENTORY_SPACING;
            let y = graphics.get_mouse_pos().1 + INVENTORY_SPACING;

            for (item, count) in &recipe.ingredients {
                render_inventory_slot(graphics, items, gfx::FloatPos(x, y), Some(&ItemStack::new(*item, *count)));
                x += INVENTORY_SLOT_SIZE + INVENTORY_SPACING;
            }
        }

        Ok(())
    }

    fn render_mouse_item(&mut self, graphics: &gfx::GraphicsContext, items: &ClientItems) {
        if self.open_state != OpenState::Closed {
            render_item_stack(graphics, items, graphics.get_mouse_pos(), self.inventory.get_selected_item().as_ref());
        }
    }

    fn render_block_ui(&mut self, graphics: &gfx::GraphicsContext, items: &ClientItems, blocks: &Blocks) -> Result<()> {
        if let OpenState::OpenedBlock { x, y } = self.open_state {
            let slots = blocks.get_block_inventory_data(x, y)?;
            let slots_pos = blocks.get_block_type_at(x, y)?.inventory_slots;
            if let Some(slots) = slots {
                for (slot, (item, pos)) in slots.iter().zip(slots_pos.iter()).enumerate() {
                    let item = item.as_ref();

                    let hovered = render_inventory_slot(
                        graphics,
                        items,
                        gfx::FloatPos(pos.0 as f32 + graphics.get_window_size().0 / 2.0 - INVENTORY_SLOT_SIZE / 2.0, pos.1 as f32),
                        item,
                    );

                    if hovered {
                        self.hovered_slot = HoveredSlot::Block(slot);
                    }
                }
            } else {
                self.open_state = OpenState::Closed;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn render(&mut self, graphics: &gfx::GraphicsContext, items: &ClientItems, networking: &mut ClientNetworking, blocks: &Blocks) -> Result<()> {
        let open_target = if self.open_state == OpenState::Closed { 0.0 } else { 1.0 };
        self.open_progress += (open_target - self.open_progress) / 5.0;

        if self.open_state == OpenState::Closed {
            if self.inventory.selected_slot.is_none() {
                self.select_slot(Some(0), networking)?;
            }

            if let Some(slot) = self.inventory.selected_slot {
                if slot >= 10 {
                    self.select_slot(Some(slot - 10), networking)?;
                }
            }
        }

        self.hovered_slot = HoveredSlot::None;
        self.render_inventory(graphics, items)?;
        self.render_block_ui(graphics, items, blocks)?;
        self.render_mouse_item(graphics, items);
        self.render_crafting(graphics, items)?;

        Ok(())
    }

    fn select_slot(&mut self, slot: Option<usize>, networking: &mut ClientNetworking) -> Result<()> {
        self.inventory.selected_slot = slot;
        let packet = InventorySelectPacket { slot };
        networking.send_packet(Packet::new(packet)?)
    }

    fn swap_slot_with_selected_slot(&mut self, slot: usize, networking: &mut ClientNetworking) -> Result<()> {
        self.inventory.swap_with_selected_item(slot)?;
        let packet = InventorySwapPacket { slot: Slot::Inventory(slot) };
        networking.send_packet(Packet::new(packet)?)
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ClientNetworking, items: &ClientItems, blocks: &mut Blocks, events: &mut EventManager) -> Result<()> {
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
                    let mut already_deselected = false;
                    if let Some(selected_slot) = self.inventory.selected_slot {
                        if let HoveredSlot::Inventory(hovered_slot) = self.hovered_slot {
                            if selected_slot == hovered_slot {
                                self.select_slot(None, networking)?;
                                already_deselected = true;
                            }
                        }
                    }

                    if !already_deselected {
                        if let HoveredSlot::Inventory(hovered_slot) = self.hovered_slot {
                            if self.inventory.get_selected_item().is_some() {
                                self.swap_slot_with_selected_slot(hovered_slot, networking)?;
                                self.select_slot(None, networking)?;
                            } else {
                                self.select_slot(Some(hovered_slot), networking)?;
                            }
                        }
                    }

                    if let HoveredSlot::Recipe(recipe) = self.hovered_slot {
                        networking.send_packet(Packet::new(InventoryCraftPacket { recipe })?)?;
                    }

                    if let HoveredSlot::Block(slot) = self.hovered_slot {
                        if self.inventory.selected_slot.is_none() {
                            let mut free_slot = None;
                            for (index, inventory_slot) in self.inventory.iter().enumerate() {
                                if inventory_slot.is_none() {
                                    free_slot = Some(index);
                                    break;
                                }
                            }

                            self.select_slot(free_slot, networking)?;
                        }

                        if let Some(selected_slot) = self.inventory.selected_slot {
                            if let OpenState::OpenedBlock { x, y } = self.open_state {
                                let inventory_item = self.inventory.get_item(selected_slot)?;
                                let mut block_inventory = blocks.get_block_inventory_data(x, y)?.ok_or_else(|| anyhow!("no block inventory"))?.clone();
                                let block_item = block_inventory.get(slot).ok_or_else(|| anyhow!("no block item"))?.clone();
                                self.inventory.set_item(selected_slot, block_item)?;
                                *block_inventory.get_mut(slot).ok_or_else(|| anyhow!("no block item"))? = inventory_item;
                                blocks.set_block_inventory_data(x, y, block_inventory, events)?;

                                let packet = InventorySwapPacket { slot: Slot::Block(x, y, slot) };
                                networking.send_packet(Packet::new(packet)?)?;
                            }
                        }
                    }
                }
                gfx::Key::E => {
                    if self.open_state == OpenState::Closed {
                        self.open_state = OpenState::Open;
                    } else {
                        self.open_state = OpenState::Closed;
                    }
                }
                _ => {}
            }
        } else if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<InventoryPacket>() {
                self.inventory.transfer_items_from(packet.inventory);
                self.update_craftable_recipes(items);
            }
        } else if let Some(event) = event.downcast::<BlockRightClickEvent>() {
            let has_inventory = !blocks.get_block_type_at(event.x, event.y)?.inventory_slots.is_empty();

            if has_inventory {
                let offset = blocks.get_block_from_main(event.x, event.y)?;

                self.open_state = OpenState::OpenedBlock {
                    x: event.x - offset.0,
                    y: event.y - offset.1,
                };
            }
        }

        Ok(())
    }
}
