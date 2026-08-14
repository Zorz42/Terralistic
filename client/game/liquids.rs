use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use anyhow::{anyhow, bail, Result};

use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::libraries::grid::ChunkTracker;
use crate::libraries::scripting::ScriptHost;
use crate::shared::blocks::RENDER_BLOCK_WIDTH;
use crate::shared::liquids::{init_liquids_mod_interface, LiquidChangeEvent, LiquidChangesPacket, LiquidId, Liquids, LiquidsWelcomePacket, MAX_LIQUID_LEVEL};
use crate::shared::packet::Packet;
use crate::shared::CHUNK_SIZE;

use super::camera::Camera;
use super::networking::WelcomePacketEvent;

const MAX_LOADED_CHUNKS: usize = 1000;

pub struct RenderLiquidChunk {
    needs_update: bool,
    rect_array: gfx::RectArray,
}

impl RenderLiquidChunk {
    pub const fn new() -> Self {
        Self {
            needs_update: true,
            rect_array: gfx::RectArray::new(),
        }
    }

    pub fn clear(&mut self) {
        self.rect_array = gfx::RectArray::new();
        self.needs_update = true;
    }

    pub fn update(&mut self, atlas: &gfx::TextureAtlas<LiquidId>, world_x: i32, world_y: i32, liquids: &Liquids) -> Result<bool> {
        if !self.needs_update {
            return Ok(false);
        }
        self.needs_update = false;

        self.rect_array = gfx::RectArray::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let liquid = liquids.get_liquid(world_x + x, world_y + y)?;
                if liquid.level == 0 {
                    continue;
                }

                let Some(src_rect) = atlas.get_rect(&liquid.id) else {
                    continue;
                };

                // A partly filled cell is drawn as a surface: the liquid sits at the bottom
                // of the cell and its top edge is where the level says. A cell with the same
                // liquid above it has no surface, so it is drawn full - otherwise every cell
                // of a settled pool that happens to hold 99 would draw a gap through it.
                let covered = liquids.get_liquid(world_x + x, world_y + y - 1).is_ok_and(|above| above.level != 0 && above.id == liquid.id);

                let fill = if covered { 1.0 } else { f32::from(liquid.level) / f32::from(MAX_LIQUID_LEVEL) };

                let mut src_rect = *src_rect;
                src_rect.pos.1 += src_rect.size.1 * (1.0 - fill);
                src_rect.size.1 *= fill;

                let dest_rect = gfx::Rect::new(
                    gfx::FloatPos(x as f32 * RENDER_BLOCK_WIDTH, (y as f32 + 1.0 - fill) * RENDER_BLOCK_WIDTH),
                    gfx::FloatSize(RENDER_BLOCK_WIDTH, fill * RENDER_BLOCK_WIDTH),
                );

                self.rect_array.add_rect(
                    &dest_rect,
                    &[
                        gfx::Color::new(255, 255, 255, 255),
                        gfx::Color::new(255, 255, 255, 255),
                        gfx::Color::new(255, 255, 255, 255),
                        gfx::Color::new(255, 255, 255, 255),
                    ],
                    &src_rect,
                );
            }
        }

        self.rect_array.update();

        Ok(true)
    }

    pub fn render(&self, graphics: &gfx::GraphicsContext, atlas: &gfx::TextureAtlas<LiquidId>, world_x: i32, world_y: i32, camera: &Camera) {
        let screen_x = world_x as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
        let screen_y = world_y as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;
        self.rect_array.render(graphics, Some(atlas.get_texture()), gfx::FloatPos(screen_x.round(), screen_y.round()));
    }
}

/// Client liquids applies what the server says the liquids are doing, and draws them.
pub struct ClientLiquids {
    liquids: Arc<Mutex<Liquids>>,
    chunks: Vec<RenderLiquidChunk>,
    atlas: gfx::TextureAtlas<LiquidId>,
    chunk_tracker: ChunkTracker,
}

impl ClientLiquids {
    pub fn new() -> Self {
        Self {
            liquids: Arc::new(Mutex::new(Liquids::new())),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
            chunk_tracker: ChunkTracker::new(0),
        }
    }

    pub fn get_liquids(&self) -> MutexGuard<'_, Liquids> {
        self.liquids.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn init(&self, mods: &mut ScriptHost) -> Result<()> {
        init_liquids_mod_interface(mods, &self.liquids)
    }

    /// This function returns the chunk index at a given chunk position
    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.get_liquids().get_size().0 as i32 / CHUNK_SIZE || y >= self.get_liquids().get_size().1 as i32 / CHUNK_SIZE {
            bail!("Tried to get liquid chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.get_liquids().get_size().0 as i32 / CHUNK_SIZE)) as usize)
    }

    pub fn on_event(&mut self, event: &Event, events: &mut EventManager) -> Result<()> {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<LiquidsWelcomePacket>() {
                self.get_liquids().deserialize(&packet.data)?;
            }
        } else if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<LiquidChangesPacket>() {
                let mut liquids = self.get_liquids();
                for change in packet.changes {
                    liquids.set_liquid(change.x, change.y, change.liquid, change.level, events)?;
                }
            }
        } else if let Some(event) = event.downcast::<LiquidChangeEvent>() {
            // the cell below is redrawn too: whether it has a surface of its own depends on
            // whether this one is filled
            for (x, y) in [(event.x, event.y), (event.x, event.y + 1)] {
                if let Ok(chunk_index) = self.get_chunk_index(x / CHUNK_SIZE, y / CHUNK_SIZE) {
                    self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?.needs_update = true;
                }
            }
        }
        Ok(())
    }

    pub fn load_resources(&mut self, mods: &ScriptHost) -> Result<()> {
        let width = self.get_liquids().get_size().0 as i32 / CHUNK_SIZE;
        let height = self.get_liquids().get_size().1 as i32 / CHUNK_SIZE;
        for _ in 0..width * height {
            self.chunks.push(RenderLiquidChunk::new());
        }

        self.chunk_tracker = ChunkTracker::new((width * height) as usize);

        // go through all the liquid types, get their images and load them
        let mut surfaces = HashMap::new();
        let liquid_ids = self.get_liquids().get_all_liquid_ids();
        for id in liquid_ids {
            let name = self.get_liquids().get_liquid_type(id)?.name.clone();
            let image_resource = mods.get_resource(&format!("liquids:{name}.opa"));
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone())?;
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        Ok(())
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, camera: &Camera) -> Result<()> {
        let width = self.get_liquids().get_size().0 as i32;
        let height = self.get_liquids().get_size().1 as i32;

        let (top_left_x, top_left_y) = camera.get_top_left(graphics);
        let (bottom_right_x, bottom_right_y) = camera.get_bottom_right(graphics);

        let (start_x, start_y) = (i32::max(0, top_left_x as i32 / CHUNK_SIZE), i32::max(0, top_left_y as i32 / CHUNK_SIZE));
        let (end_x, end_y) = (
            i32::min(width / CHUNK_SIZE, bottom_right_x as i32 / CHUNK_SIZE + 1),
            i32::min(height / CHUNK_SIZE, bottom_right_y as i32 / CHUNK_SIZE + 1),
        );

        for x in start_x..end_x {
            for y in start_y..end_y {
                let chunk_index = self.get_chunk_index(x, y)?;
                let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?;

                let liquids = self.liquids.lock().unwrap_or_else(PoisonError::into_inner);
                let has_updated = chunk.update(&self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, &liquids)?;
                drop(liquids);

                if has_updated {
                    self.chunk_tracker.update(chunk_index)?;
                }

                chunk.render(graphics, &self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, camera);
            }
        }

        while self.chunk_tracker.get_num_chunks() > MAX_LOADED_CHUNKS {
            let chunk_index = self.chunk_tracker.get_oldest_chunk()?;
            self.chunk_tracker.remove_chunk(chunk_index)?;
            self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?.clear();
        }

        Ok(())
    }
}
