use super::camera::Camera;
use super::networking::WelcomePacketEvent;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::shared::blocks::{Blocks, BLOCK_WIDTH, CHUNK_SIZE, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::{WallId, Walls, WallsWelcomePacket};
use std::collections::HashMap;

pub struct RenderWallChunk {
    needs_update: bool,
    rect_array: gfx::RectArray,
}

impl RenderWallChunk {
    pub fn new() -> Self {
        Self {
            needs_update: true,
            rect_array: gfx::RectArray::new(),
        }
    }

    fn can_connect_to(x: i32, y: i32, walls: &Walls) -> bool {
        if x < 0 || y < 0 || x >= walls.get_width() as i32 || y >= walls.get_height() as i32 {
            return true;
        }
        let wall = walls.get_wall_type_at(x, y).unwrap();
        wall.get_id() != walls.clear
    }

    pub fn render(
        &mut self,
        graphics: &mut GraphicsContext,
        atlas: &gfx::TextureAtlas<WallId>,
        world_x: i32,
        world_y: i32,
        walls: &Walls,
        camera: &Camera,
    ) {
        if self.needs_update {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let curr_wall = walls.get_wall_type_at(world_x + x, world_y + y).unwrap();
                    if let Some(curr_wall_rect) = atlas.get_rect(&curr_wall.get_id()) {
                        let mut curr_wall_rect = *curr_wall_rect;
                        let mut dest_rect = gfx::Rect::new(
                            FloatPos(
                                ((x - 1) * RENDER_BLOCK_WIDTH) as f32,
                                ((y - 1) * RENDER_BLOCK_WIDTH) as f32,
                            ),
                            FloatSize(
                                (3 * RENDER_BLOCK_WIDTH) as f32,
                                (3 * RENDER_BLOCK_WIDTH) as f32,
                            ),
                        );

                        let curr_x = world_x + x;
                        let curr_y = world_y + y;
                        if Self::can_connect_to(curr_x - 1, curr_y, walls) {
                            dest_rect.pos.0 += RENDER_BLOCK_WIDTH as f32;
                            dest_rect.size.0 -= RENDER_BLOCK_WIDTH as f32;
                            curr_wall_rect.pos.0 += BLOCK_WIDTH as f32;
                            curr_wall_rect.size.0 -= BLOCK_WIDTH as f32;
                        }

                        if Self::can_connect_to(curr_x + 1, curr_y, walls) {
                            dest_rect.size.0 -= RENDER_BLOCK_WIDTH as f32;
                            curr_wall_rect.size.0 -= BLOCK_WIDTH as f32;
                        }

                        if Self::can_connect_to(curr_x, curr_y - 1, walls) {
                            dest_rect.pos.1 += RENDER_BLOCK_WIDTH as f32;
                            dest_rect.size.1 -= RENDER_BLOCK_WIDTH as f32;
                            curr_wall_rect.pos.1 += BLOCK_WIDTH as f32;
                            curr_wall_rect.size.1 -= BLOCK_WIDTH as f32;
                        }

                        if Self::can_connect_to(curr_x, curr_y + 1, walls) {
                            dest_rect.size.1 -= RENDER_BLOCK_WIDTH as f32;
                            curr_wall_rect.size.1 -= BLOCK_WIDTH as f32;
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
        }

        let screen_x = world_x as f32 * RENDER_BLOCK_WIDTH as f32
            - camera.get_top_left(graphics).0 * RENDER_SCALE;
        let screen_y = world_y as f32 * RENDER_BLOCK_WIDTH as f32
            - camera.get_top_left(graphics).1 * RENDER_SCALE;
        self.rect_array.render(
            graphics,
            Some(atlas.get_texture()),
            FloatPos(screen_x, screen_y),
        );
    }
}

/**
client blocks handles client side block stuff, such as rendering
 */
pub struct ClientWalls {
    pub walls: Walls,
    chunks: Vec<RenderWallChunk>,
    atlas: gfx::TextureAtlas<WallId>,
    breaking_texture: gfx::Texture,
}

impl ClientWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Walls::new(blocks),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(&HashMap::new()),
            breaking_texture: gfx::Texture::new(),
        }
    }

    /**
    This function returns the chunk index at a given world position
     */
    fn get_chunk_index(&self, x: i32, y: i32) -> usize {
        // check if x and y are in bounds
        if x < 0
            || y < 0
            || x >= self.walls.get_width() as i32 / CHUNK_SIZE
            || y >= self.walls.get_height() as i32 / CHUNK_SIZE
        {
            panic!("Tried to get chunk at {x}, {y} but it is out of bounds");
        }

        (x + y * (self.walls.get_width() as i32 / CHUNK_SIZE)) as usize
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<WallsWelcomePacket>() {
                self.walls.deserialize(&packet.data).unwrap();
            }
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.walls.init(mods).unwrap();
    }

    pub fn load_resources(&mut self, mods: &mut ModManager) {
        for _ in 0..self.walls.get_width() as i32 / CHUNK_SIZE * self.walls.get_height() as i32
            / CHUNK_SIZE
        {
            self.chunks.push(RenderWallChunk::new());
        }

        // go through all the block types get their images and load them
        let mut surfaces = HashMap::new();
        for id in self.walls.get_all_wall_ids() {
            let wall_type = self.walls.get_wall_type(id).unwrap();
            let image_resource =
                mods.get_resource(format!("walls:{}.opa", wall_type.name).as_str());
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize_from_bytes(&image_resource.clone()).unwrap();
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(&surfaces);

        self.breaking_texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(mods.get_resource("misc:breaking.opa").unwrap())
                .unwrap(),
        );
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext, camera: &Camera) {
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
                    && x < self.walls.get_width() as i32 / CHUNK_SIZE
                    && y < self.walls.get_height() as i32 / CHUNK_SIZE
                {
                    let chunk_index = self.get_chunk_index(x, y);
                    let chunk = &mut self.chunks[chunk_index];

                    chunk.render(
                        graphics,
                        &self.atlas,
                        x * CHUNK_SIZE,
                        y * CHUNK_SIZE,
                        &self.walls,
                        camera,
                    );
                }
            }
        }

        for breaking_wall in self.walls.get_breaking_walls() {
            if breaking_wall.coord.0 < top_left_x
                || breaking_wall.coord.0 > bottom_right_x
                || breaking_wall.coord.1 < top_left_y
                || breaking_wall.coord.1 > bottom_right_y
            {
                continue;
            }

            let (x, y) = (
                breaking_wall.coord.0 * RENDER_BLOCK_WIDTH
                    - (camera.get_top_left(graphics).0 * RENDER_SCALE) as i32,
                breaking_wall.coord.1 * RENDER_BLOCK_WIDTH
                    - (camera.get_top_left(graphics).1 * RENDER_SCALE) as i32,
            );
            let break_stage = self
                .walls
                .get_break_stage(breaking_wall.coord.0, breaking_wall.coord.1)
                .unwrap();
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
    }

    pub fn update(&mut self, frame_length: f32, events: &mut EventManager) {
        self.walls
            .update_breaking_walls(frame_length, events)
            .unwrap();
    }
}
