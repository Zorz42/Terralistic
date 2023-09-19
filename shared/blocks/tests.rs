use crate::shared::blocks::Blocks;

#[test]
fn test_blocks_new() {
    let blocks = Blocks::new();
    assert_eq!(blocks.get_width(), 0);
    assert_eq!(blocks.get_height(), 0);
}
