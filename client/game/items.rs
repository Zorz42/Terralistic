use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::client::game::camera::Camera;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::shared::blocks::{RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::entities::{Entities, PositionComponent};
use crate::shared::items::{ItemComponent, ItemId, Items, ItemSpawnPacket};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;

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

    pub fn on_event(&mut self, event: &Event, entities: &mut Entities, events: &mut EventManager) {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<ItemSpawnPacket>() {
                self.items.spawn_item(
                    events,
                    entities,
                    packet.item_type,
                    packet.x,
                    packet.y,
                    Some(packet.id),
                );
            }
        }
    }

    pub fn render(
        &self,
        graphics: &mut gfx::GraphicsContext,
        camera: &Camera,
        entities: &mut Entities,
    ) -> Result<()> {
        for (_entity, (position, item)) in entities.ecs.query_mut::<(&PositionComponent, &ItemComponent)>() {
            let mut src_rect = *self.atlas.get_rect(&item.get_item_type()).ok_or_else(|| anyhow!("Item not found in atlas"))?;
            src_rect.size.0 /= 2.0;
            let top_left = camera.get_top_left(graphics);

            self.atlas.get_texture().render(
                &graphics.renderer,
                RENDER_SCALE,
                gfx::FloatPos(
                    position.x() * RENDER_BLOCK_WIDTH - top_left.0 * RENDER_BLOCK_WIDTH + 0.5 * RENDER_BLOCK_WIDTH - src_rect.size.0 / 2.0 * RENDER_SCALE,
                    position.y() * RENDER_BLOCK_WIDTH - top_left.1 * RENDER_BLOCK_WIDTH + 0.5 * RENDER_BLOCK_WIDTH - src_rect.size.1 / 2.0 * RENDER_SCALE,
                ),
                Some(src_rect),
                false,
                None,
            );
        }

        Ok(())
    }

    #[must_use]
    pub const fn get_atlas(&self) -> &gfx::TextureAtlas<ItemId> {
        &self.atlas
    }
}
