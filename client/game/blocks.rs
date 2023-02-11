use super::camera::Camera;
use super::networking::WelcomePacketEvent;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::shared::blocks::{
    BlockBreakStartPacket, BlockBreakStopPacket, BlockChangeEvent, BlockChangePacket, BlockId,
};
use crate::shared::blocks::{
    Blocks, BlocksWelcomePacket, BLOCK_WIDTH, CHUNK_SIZE, RENDER_BLOCK_WIDTH, RENDER_SCALE,
};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;

pub struct RenderBlockChunk {
    needs_update: bool,
    rect_array: gfx::RectArray,
}

impl RenderBlockChunk {
    pub fn new() -> Self {
        Self {
            needs_update: true,
            rect_array: gfx::RectArray::new(),
        }
    }

    fn can_connect_to(block_type: BlockId, x: i32, y: i32, blocks: &Blocks) -> bool {
        let block = blocks.get_block_type_at(x, y);
        let Ok(block) = block else { return true };
        let block_type = blocks.get_block_type(block_type);
        let Ok(block_type) = block_type else { return true };
        block_type.connects_to.contains(&block.get_id()) || block.get_id() == block_type.get_id()
    }

    pub fn render(
        &mut self,
        graphics: &mut GraphicsContext,
        atlas: &gfx::TextureAtlas<BlockId>,
        world_x: i32,
        world_y: i32,
        blocks: &Blocks,
        camera: &Camera,
    ) -> Result<()> {
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
                                if Self::can_connect_to(
                                    block_type.get_id(),
                                    world_x + x + coord.0,
                                    world_y + y + coord.1,
                                    blocks,
                                ) {
                                    block_state += 1 << i;
                                }
                            }
                        }

                        curr_block_rect.size.0 = BLOCK_WIDTH as f32;
                        curr_block_rect.size.1 = BLOCK_WIDTH as f32;
                        curr_block_rect.pos.1 += (BLOCK_WIDTH * block_state) as f32;
                        self.rect_array.add_rect(
                            &gfx::Rect::new(
                                FloatPos(
                                    (x * RENDER_BLOCK_WIDTH) as f32,
                                    (y * RENDER_BLOCK_WIDTH) as f32,
                                ),
                                FloatSize(RENDER_BLOCK_WIDTH as f32, RENDER_BLOCK_WIDTH as f32),
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
        }

        let screen_x =
            world_x * RENDER_BLOCK_WIDTH - (camera.get_top_left(graphics).0 * RENDER_SCALE) as i32;
        let screen_y =
            world_y * RENDER_BLOCK_WIDTH - (camera.get_top_left(graphics).1 * RENDER_SCALE) as i32;
        self.rect_array.render(
            graphics,
            Some(atlas.get_texture()),
            FloatPos(screen_x as f32, screen_y as f32),
        );
        Ok(())
    }
}

/**
client blocks handles client side block stuff, such as rendering
 */
pub struct ClientBlocks {
    pub blocks: Blocks,
    chunks: Vec<RenderBlockChunk>,
    atlas: gfx::TextureAtlas<BlockId>,
    breaking_texture: gfx::Texture,
}

impl ClientBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
            breaking_texture: gfx::Texture::new(),
        }
    }

    /**
    This function returns the chunk index at a given world position
     */
    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        // check if x and y are in bounds
        if x < 0
            || y < 0
            || x >= self.blocks.get_width() as i32 / CHUNK_SIZE
            || y >= self.blocks.get_height() as i32 / CHUNK_SIZE
        {
            bail!("Tried to get chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.blocks.get_width() as i32 / CHUNK_SIZE)) as usize)
    }

    pub fn on_event(&mut self, event: &Event, events: &mut EventManager) -> Result<()> {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<BlocksWelcomePacket>() {
                self.blocks.deserialize(&packet.data)?;
            }
        } else if let Some(event) = event.downcast::<Packet>() {
            if let Some(packet) = event.try_deserialize::<BlockBreakStartPacket>() {
                self.blocks
                    .start_breaking_block(events, packet.x, packet.y)?;
            } else if let Some(packet) = event.try_deserialize::<BlockBreakStopPacket>() {
                self.blocks
                    .stop_breaking_block(events, packet.x, packet.y)?;
                self.blocks
                    .set_break_progress(packet.x, packet.y, packet.break_time)?;
            } else if let Some(packet) = event.try_deserialize::<BlockChangePacket>() {
                self.blocks
                    .set_block(events, packet.x, packet.y, packet.block)?;
            }
        } else if let Some(event) = event.downcast::<BlockChangeEvent>() {
            for (x, y) in [
                (event.x, event.y),
                (event.x - 1, event.y),
                (event.x + 1, event.y),
                (event.x, event.y - 1),
                (event.x, event.y + 1),
            ] {
                let chunk_index = self.get_chunk_index(x / CHUNK_SIZE, y / CHUNK_SIZE)?;
                self.chunks
                    .get_mut(chunk_index)
                    .ok_or_else(|| anyhow!("Chunk array malformed"))?
                    .needs_update = true;
            }
        }
        Ok(())
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.blocks.init(mods)
    }

    pub fn load_resources(&mut self, mods: &mut ModManager) -> Result<()> {
        for _ in 0..self.blocks.get_width() as i32 / CHUNK_SIZE * self.blocks.get_height() as i32
            / CHUNK_SIZE
        {
            self.chunks.push(RenderBlockChunk::new());
        }

        // go through all the block types get their images and load them
        let mut surfaces = HashMap::new();
        for id in self.blocks.get_all_block_ids() {
            let block_type = self.blocks.get_block_type(id)?;
            let image_resource = mods.get_resource(&format!("blocks:{}.opa", block_type.name));
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone())?;
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        self.breaking_texture =
            gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(
                mods.get_resource("misc:breaking.opa")
                    .ok_or_else(|| anyhow!("Could not find misc:breaking.opa"))?,
            )?);
        Ok(())
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext, camera: &Camera) -> Result<()> {
        let (top_left_x, top_left_y) = camera.get_top_left(graphics);
        let (bottom_right_x, bottom_right_y) = camera.get_bottom_right(graphics);
        let (top_left_x, top_left_y) = (
            top_left_x as i32 / BLOCK_WIDTH,
            top_left_y as i32 / BLOCK_WIDTH,
        );
        let (bottom_right_x, bottom_right_y) = (
            bottom_right_x as i32 / BLOCK_WIDTH,
            bottom_right_y as i32 / BLOCK_WIDTH,
        );
        let (top_left_chunk_x, top_left_chunk_y) =
            (top_left_x / CHUNK_SIZE, top_left_y / CHUNK_SIZE);
        let (bottom_right_chunk_x, bottom_right_chunk_y) = (
            bottom_right_x / CHUNK_SIZE + 1,
            bottom_right_y / CHUNK_SIZE + 1,
        );
        for x in top_left_chunk_x..bottom_right_chunk_x {
            for y in top_left_chunk_y..bottom_right_chunk_y {
                if x >= 0
                    && y >= 0
                    && x < self.blocks.get_width() as i32 / CHUNK_SIZE
                    && y < self.blocks.get_height() as i32 / CHUNK_SIZE
                {
                    let chunk_index = self.get_chunk_index(x, y)?;
                    let chunk = self
                        .chunks
                        .get_mut(chunk_index)
                        .ok_or_else(|| anyhow!("Chunk array malformed"))?;

                    chunk.render(
                        graphics,
                        &self.atlas,
                        x * CHUNK_SIZE,
                        y * CHUNK_SIZE,
                        &self.blocks,
                        camera,
                    )?;
                }
            }
        }

        for breaking_block in self.blocks.get_breaking_blocks() {
            if breaking_block.coord.0 < top_left_x
                || breaking_block.coord.0 > bottom_right_x
                || breaking_block.coord.1 < top_left_y
                || breaking_block.coord.1 > bottom_right_y
            {
                continue;
            }

            let (x, y) = (
                breaking_block.coord.0 * RENDER_BLOCK_WIDTH
                    - (camera.get_top_left(graphics).0 * RENDER_SCALE) as i32,
                breaking_block.coord.1 * RENDER_BLOCK_WIDTH
                    - (camera.get_top_left(graphics).1 * RENDER_SCALE) as i32,
            );
            let break_stage = self
                .blocks
                .get_break_stage(breaking_block.coord.0, breaking_block.coord.1)?;
            self.breaking_texture.render(
                &graphics.renderer,
                RENDER_SCALE,
                FloatPos(x as f32, y as f32),
                Some(gfx::Rect::new(
                    FloatPos(0.0, break_stage as f32 * 8.0),
                    FloatSize(8.0, 8.0),
                )),
                false,
                None,
            );
        }

        Ok(())
    }

    pub fn update(&mut self, frame_length: f32, events: &mut EventManager) -> Result<()> {
        self.blocks.update_breaking_blocks(events, frame_length)
    }
}
