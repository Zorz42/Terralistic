use std::collections::HashMap;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::client::game::block_selector::BlockRightClickEvent;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::libraries::grid::ChunkTracker;
use crate::libraries::scripting::ScriptHost;
use crate::shared::blocks::{init_blocks_mod_interface, BlockBreakStartPacket, BlockBreakStopPacket, BlockChangeEvent, BlockChangePacket, BlockId, BlockRightClickPacket};
use crate::shared::blocks::{Blocks, BlocksWelcomePacket, BLOCK_WIDTH, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::packet::Packet;
use crate::shared::CHUNK_SIZE;
use anyhow::{anyhow, bail, Result};

use super::camera::Camera;
use super::networking::{ClientNetworking, WelcomePacketEvent};

const MAX_LOADED_CHUNKS: usize = 1000;

pub struct RenderBlockChunk {
    needs_update: bool,
    rect_array: gfx::RectArray,
}

impl RenderBlockChunk {
    pub const fn new() -> Self {
        Self {
            needs_update: true,
            rect_array: gfx::RectArray::new(),
        }
    }

    // to reduce memory consumption
    pub fn clear(&mut self) {
        self.rect_array = gfx::RectArray::new();
        self.needs_update = true;
    }

    fn can_connect_to(block_type: BlockId, x: i32, y: i32, blocks: &Blocks) -> bool {
        let block = blocks.get_block_type_at(x, y);
        let Ok(block) = block else {
            return true;
        };
        let block_type = blocks.get_block_type(block_type);
        let Ok(block_type) = block_type else {
            return true;
        };
        block_type.connects_to.contains(&block.get_id()) || block.get_id() == block_type.get_id()
    }

    pub fn update(&mut self, atlas: &gfx::TextureAtlas<BlockId>, world_x: i32, world_y: i32, blocks: &Blocks) -> Result<bool> {
        if self.needs_update {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let curr_block = blocks.get_block_type_at(world_x + x, world_y + y)?;
                    if let Some(curr_block_rect) = atlas.get_rect(&curr_block.get_id()) {
                        let mut curr_block_rect = *curr_block_rect;
                        let mut block_state = 0;
                        let block_type = blocks.get_block_type_at(world_x + x, world_y + y)?;

                        if block_type.can_update_states {
                            let coordinates = [(0, -1), (1, 0), (0, 1), (-1, 0)];

                            for (i, coord) in coordinates.iter().enumerate() {
                                if Self::can_connect_to(block_type.get_id(), world_x + x + coord.0, world_y + y + coord.1, blocks) {
                                    block_state += 1 << i;
                                }
                            }
                        }

                        curr_block_rect.size.0 = BLOCK_WIDTH;
                        curr_block_rect.size.1 = BLOCK_WIDTH;
                        curr_block_rect.pos.1 += BLOCK_WIDTH * block_state as f32;

                        if block_type.width != 0 && block_type.height != 0 {
                            let from_main = blocks.get_block_from_main(world_x + x, world_y + y)?;
                            curr_block_rect.pos.0 += BLOCK_WIDTH * from_main.0 as f32;
                            curr_block_rect.pos.1 += BLOCK_WIDTH * from_main.1 as f32;
                        }

                        self.rect_array.add_rect(
                            &gfx::Rect::new(
                                gfx::FloatPos(x as f32 * RENDER_BLOCK_WIDTH, y as f32 * RENDER_BLOCK_WIDTH),
                                gfx::FloatSize(RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH),
                            ),
                            &[
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                            ],
                            &curr_block_rect,
                        );
                    }
                }
            }

            self.rect_array.update();

            return Ok(true);
        }

        Ok(false)
    }

    pub fn render(&self, graphics: &gfx::GraphicsContext, atlas: &gfx::TextureAtlas<BlockId>, world_x: i32, world_y: i32, camera: &Camera) {
        let screen_x = world_x as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
        let screen_y = world_y as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;
        self.rect_array.render(graphics, Some(atlas.get_texture()), gfx::FloatPos(screen_x.round(), screen_y.round()));
    }
}

/// client blocks handles client side block stuff, such as rendering
pub struct ClientBlocks {
    blocks: Arc<Mutex<Blocks>>,
    chunks: Vec<RenderBlockChunk>,
    atlas: gfx::TextureAtlas<BlockId>,
    breaking_texture: gfx::Texture,
    chunk_tracker: ChunkTracker,
}

impl ClientBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Arc::new(Mutex::new(Blocks::new())),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
            breaking_texture: gfx::Texture::new(),
            chunk_tracker: ChunkTracker::new(0),
        }
    }

    pub fn get_blocks(&self) -> MutexGuard<'_, Blocks> {
        self.blocks.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// This function returns the chunk index at a given world position
    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        // check if x and y are in bounds
        if x < 0 || y < 0 || x >= self.get_blocks().get_size().0 as i32 / CHUNK_SIZE || y >= self.get_blocks().get_size().1 as i32 / CHUNK_SIZE {
            bail!("Tried to get block chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.get_blocks().get_size().0 as i32 / CHUNK_SIZE)) as usize)
    }

    pub fn on_event(&mut self, event: &Event, events: &mut EventManager, networking: &mut ClientNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<BlocksWelcomePacket>() {
                self.get_blocks().deserialize(&packet.data)?;
            }
        } else if let Some(event) = event.downcast::<Packet>() {
            if let Some(packet) = event.try_deserialize::<BlockBreakStartPacket>() {
                self.get_blocks().start_breaking_block(events, packet.x, packet.y, packet.tool, packet.tool_power)?;
            } else if let Some(packet) = event.try_deserialize::<BlockBreakStopPacket>() {
                self.get_blocks().stop_breaking_block(events, packet.x, packet.y)?;
                self.get_blocks().set_break_progress(packet.x, packet.y, packet.break_time)?;
            } else if let Some(packet) = event.try_deserialize::<BlockChangePacket>() {
                self.get_blocks().set_big_block(events, packet.x, packet.y, packet.block, (packet.from_main_x, packet.from_main_y))?;
                self.get_blocks().set_block_inventory_data(packet.x, packet.y, packet.inventory, events)?;
            }
        } else if let Some(event) = event.downcast::<BlockChangeEvent>() {
            for (x, y) in [(event.x, event.y), (event.x - 1, event.y), (event.x + 1, event.y), (event.x, event.y - 1), (event.x, event.y + 1)] {
                let chunk_index = self.get_chunk_index(x / CHUNK_SIZE, y / CHUNK_SIZE)?;
                self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?.needs_update = true;
            }
        } else if let Some(event) = event.downcast::<BlockRightClickEvent>() {
            let packet = Packet::new(BlockRightClickPacket { x: event.x, y: event.y })?;
            networking.send_packet(packet)?;
        }
        Ok(())
    }

    pub fn init(&self, mods: &mut ScriptHost) -> Result<()> {
        init_blocks_mod_interface(&self.blocks, mods)?;
        Ok(())
    }

    pub fn load_resources(&mut self, mods: &ScriptHost) -> Result<()> {
        let width = self.get_blocks().get_size().0 as i32 / CHUNK_SIZE;
        let height = self.get_blocks().get_size().1 as i32 / CHUNK_SIZE;
        for _ in 0..width * height {
            self.chunks.push(RenderBlockChunk::new());
        }

        self.chunk_tracker = ChunkTracker::new((width * height) as usize);

        // go through all the block types get their images and load them
        let mut surfaces = HashMap::new();
        let all_block_ids = self.get_blocks().get_all_block_ids();
        for id in all_block_ids {
            let image_resource = mods.get_resource(&format!("blocks:{}.opa", self.get_blocks().get_block_type(id)?.name));
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone())?;
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        self.breaking_texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(
            mods.get_resource("misc:breaking.opa").ok_or_else(|| anyhow!("Could not find misc:breaking.opa"))?,
        )?);
        Ok(())
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, camera: &Camera) -> Result<()> {
        let width = self.get_blocks().get_size().0 as i32;
        let height = self.get_blocks().get_size().1 as i32;

        let (top_left_x, top_left_y) = camera.get_top_left(graphics);
        let (bottom_right_x, bottom_right_y) = camera.get_bottom_right(graphics);

        let (start_x, start_y) = (i32::max(0, top_left_x as i32 / CHUNK_SIZE), i32::max(0, top_left_y as i32 / CHUNK_SIZE));
        let (end_x, end_y) = (
            i32::min(width / CHUNK_SIZE, bottom_right_x as i32 / CHUNK_SIZE + 1),
            i32::min(height / CHUNK_SIZE, bottom_right_y as i32 / CHUNK_SIZE + 1),
        );

        let extended_view_distance = 5;
        let (extended_start_x, extended_start_y) = (i32::max(0, start_x - extended_view_distance), i32::max(0, start_y - extended_view_distance));
        let (extended_end_x, extended_end_y) = (
            i32::min(width / CHUNK_SIZE, end_x + extended_view_distance),
            i32::min(height / CHUNK_SIZE, end_y + extended_view_distance),
        );

        for x in extended_start_x..extended_end_x {
            for y in extended_start_y..extended_end_y {
                let chunk_index = self.get_chunk_index(x, y)?;
                let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?;

                let blocks = self.blocks.lock().unwrap_or_else(PoisonError::into_inner);
                let has_updated = chunk.update(&self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, &blocks)?;

                drop(blocks);

                if has_updated {
                    self.chunk_tracker.update(chunk_index)?;
                }
            }
        }

        for x in start_x..end_x {
            for y in start_y..end_y {
                let chunk_index = self.get_chunk_index(x, y)?;
                let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?;

                chunk.render(graphics, &self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, camera);
            }
        }

        while self.chunk_tracker.get_num_chunks() > MAX_LOADED_CHUNKS {
            let chunk_index = self.chunk_tracker.get_oldest_chunk()?;
            self.chunk_tracker.remove_chunk(chunk_index)?;
            self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?.clear();
        }

        let breaking_blocks = {
            let temp = self.get_blocks();
            temp.get_breaking_blocks().clone()
        };
        for breaking_block in breaking_blocks {
            if breaking_block.coord.0 < top_left_x as i32
                || breaking_block.coord.0 > bottom_right_x as i32
                || breaking_block.coord.1 < top_left_y as i32
                || breaking_block.coord.1 > bottom_right_y as i32
            {
                continue;
            }

            let (x, y) = (
                breaking_block.coord.0 as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH,
                breaking_block.coord.1 as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH,
            );
            let break_stage = self.get_blocks().get_break_stage(breaking_block.coord.0, breaking_block.coord.1)?;
            self.breaking_texture.render(
                graphics,
                RENDER_SCALE,
                gfx::FloatPos(x, y),
                Some(gfx::Rect::new(gfx::FloatPos(0.0, break_stage as f32 * 8.0), gfx::FloatSize(8.0, 8.0))),
                false,
                None,
            );
        }

        Ok(())
    }

    /// Per simulation tick, not per frame: breaking has to land on the same tick the server
    /// breaks it on, and a frame is however long the last one took.
    pub fn tick(&self, events: &mut EventManager) -> Result<()> {
        self.get_blocks().update_breaking_blocks(events, 1)
    }
}
