use crate::client::game::camera::Camera;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize};
use crate::shared::blocks::{BlockChangeEvent, BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::items::{ItemId, Items};
use crate::shared::mod_manager::ModManager;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct ClientItems {
    pub items: Items,
    atlas: gfx::TextureAtlas<ItemId>,
}

impl ClientItems {
    pub fn new() -> Self {
        Self {
            items: Items::new(),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.items.init(mods)
    }

    pub fn load_resources(&mut self, mods: &mut ModManager) -> Result<()> {
        // go through all the item types get their images and load them
        let mut surfaces = HashMap::new();
        for id in self.items.get_all_item_type_ids() {
            let item_type = self.items.get_item_type(id)?;
            let image_resource = mods.get_resource(&format!("items:{}.opa", item_type.name));
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone())?;
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event) -> Result<()> {
        if let Some(event) = event.downcast::<BlockChangeEvent>() {
            println!("Block broken at {}, {}", event.x, event.y);
            let broken_block = event.prev_block;
            let drop = self.items.get_block_drop(broken_block);
            if let Ok(drop) = drop {
                println!("Dropping item");
                // spawn item at random chance. drop.chance is a float between 0 and 1
                if rand::random::<f32>() < drop.chance {
                    println!("Dropping item by chance");
                    self.items.spawn_item(
                        &self.items.get_item_type(drop.item)?,
                        (event.x * BLOCK_WIDTH) as f32,
                        (event.y * BLOCK_WIDTH) as f32,
                        None,
                    );
                }
            }
        }
        Ok(())
    }

    pub fn render(&self, graphics: &mut gfx::GraphicsContext, camera: &Camera) -> Result<()> {
        for item in self.items.get_all_map_items() {
            let mut src_rect = *self
                .atlas
                .get_rect(&item.item_id)
                .ok_or_else(|| anyhow!("Item not found in atlas"))?;
            src_rect.pos.0 += 1.0;
            src_rect.pos.1 += 2.0;
            src_rect.size = FloatSize(8.0, 8.0);
            let top_left = camera.get_top_left(graphics);

            self.atlas.get_texture().render(
                &graphics.renderer,
                RENDER_SCALE,
                FloatPos(
                    (item.entity.x - top_left.0) * RENDER_SCALE,
                    (item.entity.y - top_left.1) * RENDER_SCALE,
                ),
                Some(src_rect),
                false,
                None,
            );
        }

        Ok(())
    }
}
