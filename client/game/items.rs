use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use anyhow::{anyhow, Result};

use crate::client::game::camera::Camera;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::shared::blocks::{RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::entities::{Entities, PhysicsComponent, PositionComponent};
use crate::shared::items::{
    init_items_mod_interface, ItemComponent, ItemId, ItemSpawnPacket, Items,
};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;

pub struct ClientItems {
    items: Arc<Mutex<Items>>,
    atlas: gfx::TextureAtlas<ItemId>,
}

impl ClientItems {
    pub fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(Items::new())),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        init_items_mod_interface(&self.items, mods)
    }

    pub fn load_resources(&mut self, mods: &ModManager) -> Result<()> {
        // go through all the item types get their images and load them
        let mut surfaces = HashMap::new();
        let item_ids = self.get_items().get_all_item_type_ids();
        for id in item_ids {
            let item_type = self.get_items().get_item_type(id)?;
            let image_resource = mods.get_resource(&format!("items:{}.opa", item_type.name));
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone())?;
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        Ok(())
    }

    pub fn on_event(
        &mut self,
        event: &Event,
        entities: &mut Entities,
        events: &mut EventManager,
    ) -> Result<()> {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<ItemSpawnPacket>() {
                let item = self.get_items().spawn_item(
                    events,
                    entities,
                    packet.item_type,
                    packet.x,
                    packet.y,
                    packet.id,
                )?;

                let mut physics = entities.ecs.get::<&mut PhysicsComponent>(item)?;
                physics.velocity_x = packet.velocity_x;
                physics.velocity_y = packet.velocity_y;
            }
        }
        Ok(())
    }

    pub fn render(
        &self,
        graphics: &gfx::GraphicsContext,
        camera: &Camera,
        entities: &mut Entities,
    ) -> Result<()> {
        for (_entity, (position, item)) in entities
            .ecs
            .query_mut::<(&PositionComponent, &ItemComponent)>()
        {
            let mut src_rect = *self
                .atlas
                .get_rect(&item.get_item_type())
                .ok_or_else(|| anyhow!("Item not found in atlas"))?;
            src_rect.size.0 /= 2.0;
            let top_left = camera.get_top_left(graphics);

            self.atlas.get_texture().render(
                &graphics.renderer,
                RENDER_SCALE,
                gfx::FloatPos(
                    (position.x() * RENDER_BLOCK_WIDTH - top_left.0 * RENDER_BLOCK_WIDTH
                        + 0.5 * RENDER_BLOCK_WIDTH
                        - src_rect.size.0 / 2.0 * RENDER_SCALE)
                        .round(),
                    (position.y() * RENDER_BLOCK_WIDTH - top_left.1 * RENDER_BLOCK_WIDTH
                        + 0.5 * RENDER_BLOCK_WIDTH
                        - src_rect.size.1 / 2.0 * RENDER_SCALE)
                        .round(),
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

    pub fn get_items(&self) -> MutexGuard<Items> {
        self.items.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
