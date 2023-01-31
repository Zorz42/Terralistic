use crate::game::camera::Camera;
use crate::game::networking::WelcomePacketEvent;
use events::Event;
use graphics as gfx;
use graphics::GraphicsContext;
use shared::blocks::BlockId;
use shared::blocks::{
    Blocks, BlocksWelcomePacket, BLOCK_WIDTH, CHUNK_SIZE, RENDER_BLOCK_WIDTH, RENDER_SCALE,
};
use shared::mod_manager::ModManager;
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
        if x < 0 || y < 0 || x >= blocks.get_width() || y >= blocks.get_height() {
            return true;
        }
        let block = blocks.get_block_type_at(x, y).unwrap();
        let block_type = blocks.get_block_type(block_type).unwrap();
        block_type.connects_to.contains(&block.get_id()) || block.get_id() == block_type.get_id()
    }

    pub fn render(
        &mut self, graphics: &mut GraphicsContext, atlas: &gfx::TextureAtlas<BlockId>,
        world_x: i32, world_y: i32, blocks: &Blocks, camera: &Camera,
    ) {
        if self.needs_update {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let curr_block = blocks.get_block_type_at(world_x + x, world_y + y).unwrap();
                    if let Some(curr_block_rect) = atlas.get_rect(curr_block.get_id()) {
                        let mut curr_block_rect = *curr_block_rect;
                        let mut block_state = 0;
                        let block_type =
                            blocks.get_block_type_at(world_x + x, world_y + y).unwrap();

                        if block_type.can_update_states {
                            let coordinates = [(0, -1), (1, 0), (0, 1), (-1, 0)];

                            for i in 0..4 {
                                if Self::can_connect_to(
                                    block_type.get_id(),
                                    world_x + x + coordinates[i].0,
                                    world_y + y + coordinates[i].1,
                                    blocks,
                                ) {
                                    block_state += 1 << i;
                                }
                            }
                        }

                        curr_block_rect.w = BLOCK_WIDTH;
                        curr_block_rect.h = BLOCK_WIDTH;
                        curr_block_rect.y += BLOCK_WIDTH * block_state;
                        self.rect_array.add_rect(
                            &gfx::Rect::new(
                                x * RENDER_BLOCK_WIDTH,
                                y * RENDER_BLOCK_WIDTH,
                                RENDER_BLOCK_WIDTH,
                                RENDER_BLOCK_WIDTH,
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
        self.rect_array
            .render(graphics, Some(atlas.get_texture()), screen_x, screen_y);
    }
}

/**
Client blocks handles client side block stuff, such as rendering
 */
pub struct ClientBlocks {
    pub blocks: Blocks,
    chunks: Vec<RenderBlockChunk>,
    atlas: gfx::TextureAtlas<BlockId>,
}

impl ClientBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
            chunks: Vec::new(),
            atlas: gfx::TextureAtlas::new(HashMap::new()),
        }
    }

    /**
    This function returns the chunk index at a given world position
     */
    fn get_chunk_index(&self, x: i32, y: i32) -> usize {
        // check if x and y are in bounds
        if x < 0
            || y < 0
            || x >= self.blocks.get_width() / CHUNK_SIZE
            || y >= self.blocks.get_height() / CHUNK_SIZE
        {
            panic!("Tried to get chunk at {}, {} but it is out of bounds", x, y);
        }

        (x + y * (self.blocks.get_width() / CHUNK_SIZE)) as usize
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.deserialize::<BlocksWelcomePacket>() {
                self.blocks.create(packet.width, packet.height);
                self.blocks.deserialize(&packet.data);
            }
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.blocks.init(mods);
    }

    pub fn load_resources(&mut self, mods: &mut ModManager) {
        for _ in 0..self.blocks.get_width() / CHUNK_SIZE * self.blocks.get_height() / CHUNK_SIZE {
            self.chunks.push(RenderBlockChunk::new());
        }

        // go through all the block types get their images and load them
        let mut surfaces = HashMap::new();
        for id in self.blocks.get_all_block_ids() {
            let block_type = self.blocks.get_block_type_by_id(id);
            let image_resource = mods.get_resource(format!("blocks:{}.opa", block_type.name));
            if let Some(image_resource) = image_resource {
                let image = gfx::Surface::deserialize(image_resource.clone());
                surfaces.insert(id, image);
            }
        }

        self.atlas = gfx::TextureAtlas::new(surfaces);
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
        let (top_left_x, top_left_y) = (top_left_x / CHUNK_SIZE, top_left_y / CHUNK_SIZE);
        let (bottom_right_x, bottom_right_y) = (
            bottom_right_x / CHUNK_SIZE + 1,
            bottom_right_y / CHUNK_SIZE + 1,
        );
        for x in top_left_x..bottom_right_x {
            for y in top_left_y..bottom_right_y {
                if x >= 0
                    && y >= 0
                    && x < self.blocks.get_width() / CHUNK_SIZE
                    && y < self.blocks.get_height() / CHUNK_SIZE
                {
                    let chunk_index = self.get_chunk_index(x, y);
                    let chunk = &mut self.chunks[chunk_index];

                    chunk.render(
                        graphics,
                        &self.atlas,
                        x * CHUNK_SIZE,
                        y * CHUNK_SIZE,
                        &self.blocks,
                        camera,
                    );
                }
            }
        }
    }
}
