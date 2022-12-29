use std::any::Any;
use std::ptr::hash;
use std::rc::Rc;
use graphics::GraphicsContext;
use graphics as gfx;
use shared::blocks::{BLOCK_WIDTH, Blocks, BlocksWelcomePacket, CHUNK_SIZE, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use shared::packet::{Packet, WelcomeCompletePacket};
use crate::game::camera::Camera;
use crate::game::networking::WelcomePacketEvent;

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

    pub fn render(&mut self, graphics: &mut GraphicsContext, atlas: &gfx::TextureAtlas, world_x: i32, world_y: i32, blocks: &Blocks, camera: &Camera) {
        if self.needs_update {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let curr_block = blocks.get_block_type(world_x + x, world_y + y);
                    if curr_block.image.get_width() != 0 && curr_block.image.get_height() != 0 {
                        self.rect_array.add_rect(
                            &gfx::Rect::new(x * RENDER_BLOCK_WIDTH, y * RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH),
                            &[
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                                gfx::Color::new(255, 255, 255, 255),
                            ],
                            &atlas.get_rect(curr_block.get_id() as usize),
                        );
                    }
                }
            }

            self.rect_array.update();
        }

        let screen_x = world_x * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 as i32;
        let screen_y = world_y * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 as i32;
        self.rect_array.render(graphics, Some(atlas.get_texture()), screen_x, screen_y);
    }
}

/**
Client blocks handles client side block stuff, such as rendering
 */
pub struct ClientBlocks {
    pub blocks: Blocks,
    chunks: Vec<RenderBlockChunk>,
    atlas: gfx::TextureAtlas,
}

impl ClientBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(Vec::new()),
        }
    }

    /**
    This function returns the chunk index at a given world position
     */
    fn get_chunk_index(&self, x: i32, y: i32) -> usize {
        // check if x and y are in bounds
        if x < 0 || y < 0 || x >= self.blocks.get_width() as i32 / CHUNK_SIZE || y >= self.blocks.get_height() as i32 / CHUNK_SIZE {
            panic!("Tried to get chunk at {}, {} but it is out of bounds", x, y);
        }

        (x + y * (self.blocks.get_width() / CHUNK_SIZE) as i32) as usize
    }

    pub fn on_event(&mut self, event: &Box<dyn Any>) {
        if let Some(event) = event.downcast_ref::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.deserialize::<BlocksWelcomePacket>() {
                self.blocks.create(packet.width, packet.height);
                self.blocks.deserialize(packet.data.clone());
            }
        }
    }

    pub fn init(&mut self) {
        let mut surfaces = Vec::new();
        for block_type in 0..self.blocks.get_number_block_types() {
            surfaces.push(self.blocks.get_block_type_by_id(block_type).image.clone());
        }
        self.atlas = gfx::TextureAtlas::new(surfaces);

        for _ in 0..self.blocks.get_width() / CHUNK_SIZE * self.blocks.get_height() / CHUNK_SIZE {
            self.chunks.push(RenderBlockChunk::new());
        }
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext, camera: &Camera) {
        let (top_left_x, top_left_y) = camera.get_top_left(graphics);
        let (bottom_right_x, bottom_right_y) = camera.get_bottom_right(graphics);
        let (top_left_x, top_left_y) = (top_left_x as i32 / RENDER_BLOCK_WIDTH, top_left_y as i32 / RENDER_BLOCK_WIDTH);
        let (bottom_right_x, bottom_right_y) = (bottom_right_x as i32 / RENDER_BLOCK_WIDTH, bottom_right_y as i32 / RENDER_BLOCK_WIDTH);
        let (top_left_x, top_left_y) = (top_left_x / CHUNK_SIZE, top_left_y / CHUNK_SIZE);
        let (bottom_right_x, bottom_right_y) = (bottom_right_x / CHUNK_SIZE + 1, bottom_right_y / CHUNK_SIZE + 1);
        for x in top_left_x..bottom_right_x {
            for y in top_left_y..bottom_right_y {
                if x >= 0 && y >= 0 && x < self.blocks.get_width() as i32 / CHUNK_SIZE && y < self.blocks.get_height() as i32 / CHUNK_SIZE {
                    let chunk_index = self.get_chunk_index(x, y);
                    let chunk = &mut self.chunks[chunk_index];

                    chunk.render(graphics, &self.atlas, x * CHUNK_SIZE, y * CHUNK_SIZE, &self.blocks, camera);
                }
            }
        }
    }
}