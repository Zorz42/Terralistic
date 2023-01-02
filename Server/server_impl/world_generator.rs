use shared::blocks::blocks::Blocks;

pub fn generate_world(blocks: &mut Blocks) {
    blocks.create(1024, 1024);
}