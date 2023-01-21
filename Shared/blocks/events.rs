//TODO: write tests for block changes, block data, and block updates

/**
Event that is fired when a block is changed
 */
pub struct BlockChangeEvent {
    pub x: i32, pub y: i32
}
impl BlockChangeEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockChangeEvent{x, y} }
}
//impl Event for BlockChangeEvent {}

/**
Event that is fired when a random tick is fired for a block
 */
struct BlockRandomTickEvent {
    x: i32, y: i32
}
impl BlockRandomTickEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockRandomTickEvent{x, y} }
}
//impl Event for BlockRandomTickEvent {}

/**
Event that is fired when a block is updated
 */
pub struct BlockUpdateEvent {
    x: i32, y: i32
}
impl BlockUpdateEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockUpdateEvent{x, y} }
}
//impl Event for BlockUpdateEvent {}