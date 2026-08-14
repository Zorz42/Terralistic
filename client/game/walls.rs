use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::libraries::grid::ChunkTracker;
use crate::libraries::timing::Budget;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::{init_walls_mod_interface, WallId, Walls, WallsWelcomePacket};
use crate::shared::CHUNK_SIZE;
use anyhow::{anyhow, bail, Result};

use super::camera::Camera;
use super::networking::WelcomePacketEvent;

const MAX_LOADED_CHUNKS: usize = 1000;

pub struct RenderWallChunk {
    needs_update: bool,
    rect_array: gfx::RectArray,
}

impl RenderWallChunk {
    pub const fn new() -> Self {
        Self {
            needs_update: true,
            rect_array: gfx::RectArray::new(),
        }
    }

    fn can_connect_to(x: i32, y: i32, walls: &Walls) -> bool {
        let wall = walls.get_wall_type_at(x, y);
        wall.map_or(true, |wall2| wall2.get_id() != walls.clear)
    }

    pub fn clear(&mut self) {
        self.rect_array = gfx::RectArray::new();
        self.needs_update = true;
    }

    pub fn update(&mut self, atlas: &gfx::TextureAtlas<WallId>, world_x: i32, world_y: i32, walls: &Walls, budget: &Budget) -> Result<bool> {
        if self.needs_update && budget.has_time_left() {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let curr_wall = walls.get_wall_type_at(world_x + x, world_y + y)?;
                    if let Some(curr_wall_rect) = atlas.get_rect(&curr_wall.get_id()) {
                        let mut curr_wall_rect = *curr_wall_rect;
                        let mut dest_rect = gfx::Rect::new(
                            gfx::FloatPos((x - 1) as f32 * RENDER_BLOCK_WIDTH, (y - 1) as f32 * RENDER_BLOCK_WIDTH),
                            gfx::FloatSize(3.0 * RENDER_BLOCK_WIDTH, 3.0 * RENDER_BLOCK_WIDTH),
                        );

                        let curr_x = world_x + x;
                        let curr_y = world_y + y;
                        if Self::can_connect_to(curr_x - 1, curr_y, walls) {
                            dest_rect.pos.0 += RENDER_BLOCK_WIDTH;
                            dest_rect.size.0 -= RENDER_BLOCK_WIDTH;
                            curr_wall_rect.pos.0 += BLOCK_WIDTH;
                            curr_wall_rect.size.0 -= BLOCK_WIDTH;
                        }

                        if Self::can_connect_to(curr_x + 1, curr_y, walls) {
                            dest_rect.size.0 -= RENDER_BLOCK_WIDTH;
                            curr_wall_rect.size.0 -= BLOCK_WIDTH;
                        }

                        if Self::can_connect_to(curr_x, curr_y - 1, walls) {
                            dest_rect.pos.1 += RENDER_BLOCK_WIDTH;
                            dest_rect.size.1 -= RENDER_BLOCK_WIDTH;
                            curr_wall_rect.pos.1 += BLOCK_WIDTH;
                            curr_wall_rect.size.1 -= BLOCK_WIDTH;
                        }

                        if Self::can_connect_to(curr_x, curr_y + 1, walls) {
                            dest_rect.size.1 -= RENDER_BLOCK_WIDTH;
                            curr_wall_rect.size.1 -= BLOCK_WIDTH;
                        }

                        self.rect_array.add_rect(
                            &dest_rect,
                            &[
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                            ],
                            &curr_wall_rect,
                        );
                    }
                }
            }

            self.rect_array.update();

            return Ok(true);
        }

        Ok(false)
    }

    pub fn render(&self, graphics: &gfx::GraphicsContext, atlas: &gfx::TextureAtlas<WallId>, world_x: i32, world_y: i32, camera: &Camera) {
        let screen_x = world_x as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
        let screen_y = world_y as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;
        self.rect_array.render(graphics, Some(atlas.get_texture()), gfx::FloatPos(screen_x.round(), screen_y.round()));
    }
}

/// Client blocks handles client side block stuff, such as rendering.
pub struct ClientWalls {
    walls: Arc<Mutex<Walls>>,
    chunks: Vec<RenderWallChunk>,
    atlas: gfx::TextureAtlas<WallId>,
    breaking_texture: gfx::Texture,
    chunk_tracker: ChunkTracker,
}

impl ClientWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Arc::new(Mutex::new(Walls::new(blocks))),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
            breaking_texture: gfx::Texture::new(),
            chunk_tracker: ChunkTracker::new(0),
        }
    }

    pub fn get_walls(&self) -> MutexGuard<'_, Walls> {
        self.walls.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// This function returns the chunk index at a given world position
    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        // check if x and y are in bounds
        if x < 0 || y < 0 || x >= self.get_walls().get_size().0 as i32 / CHUNK_SIZE || y >= self.get_walls().get_size().1 as i32 / CHUNK_SIZE {
            bail!("Tried to get wall chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.get_walls().get_size().0 as i32 / CHUNK_SIZE)) as usize)
    }

    pub fn on_event(&self, event: &Event) -> Result<()> {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<WallsWelcomePacket>() {
                self.get_walls().deserialize(&packet.data)?;
            }
        }
        Ok(())
    }

    pub fn init(&self, mods: &mut ModManager) -> Result<()> {
        init_walls_mod_interface(mods, &self.walls)
    }

    pub fn load_resources(&mut self, mods: &ModManager) -> Result<()> {
        let walls_width = self.get_walls().get_size().0 as i32;
        let walls_height = self.get_walls().get_size().1 as i32;
        let chunk_count = (walls_width / CHUNK_SIZE * walls_height / CHUNK_SIZE) as usize;
        for _ in 0..chunk_count {
            self.chunks.push(RenderWallChunk::new());
        }

        self.chunk_tracker = ChunkTracker::new(chunk_count);

        // go through all the block types get their images and load them
        let mut surfaces = HashMap::new();
        let wall_ids = self.get_walls().get_all_wall_ids();
        for id in wall_ids {
            let wall_type = self.get_walls().get_wall_type(id)?;
            let image_resource = mods.get_resource(format!("walls:{}.opa", wall_type.name).as_str());
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone())?;
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        self.breaking_texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(
            mods.get_resource("misc:breaking.opa").ok_or_else(|| anyhow!("could not get misc:breaking.opa resource"))?,
        )?);

        Ok(())
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, camera: &Camera, budget: &Budget) -> Result<()> {
        let width = self.get_walls().get_size().0 as i32;
        let height = self.get_walls().get_size().1 as i32;

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
                let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("chunks array malformed"))?;
                let walls = self.walls.lock().unwrap_or_else(PoisonError::into_inner);

                let has_updated = chunk.update(&self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, &walls, budget)?;
                if has_updated {
                    self.chunk_tracker.update(chunk_index)?;
                }
            }
        }

        for x in start_x..end_x {
            for y in start_y..end_y {
                if x >= 0 && y >= 0 && x < self.get_walls().get_size().0 as i32 / CHUNK_SIZE && y < self.get_walls().get_size().1 as i32 / CHUNK_SIZE {
                    let chunk_index = self.get_chunk_index(x, y)?;
                    let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("chunks array malformed"))?;

                    chunk.render(graphics, &self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, camera);
                }
            }
        }

        while self.chunk_tracker.get_num_chunks() > MAX_LOADED_CHUNKS {
            let chunk_index = self.chunk_tracker.get_oldest_chunk()?;
            self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("chunks array malformed"))?.clear();
            self.chunk_tracker.remove_chunk(chunk_index)?;
        }

        let walls = self.get_walls();
        let breaking_walls = walls.get_breaking_walls();
        for breaking_wall in breaking_walls {
            if breaking_wall.coord.0 < top_left_x as i32 || breaking_wall.coord.0 > bottom_right_x as i32 || breaking_wall.coord.1 < top_left_y as i32 || breaking_wall.coord.1 > bottom_right_y as i32
            {
                continue;
            }

            let (x, y) = (
                breaking_wall.coord.0 as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH,
                breaking_wall.coord.1 as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH,
            );
            let break_stage = self.get_walls().get_break_stage(breaking_wall.coord.0, breaking_wall.coord.1)?;
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

    pub fn update(&self, frame_length: f32, events: &mut EventManager) -> Result<()> {
        self.get_walls().update_breaking_walls(frame_length, events)
    }
}
