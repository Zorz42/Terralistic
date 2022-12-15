use super::blocks::*;
use events::*;

struct WallChangeEvent {
    pub x: i32,
    pub y: i32
}
impl WallChangeEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallChangeEvent{ x, y } }
}
impl Event for WallChangeEvent {}

struct WallBreakEvent {
    pub x: i32,
    pub y: i32
}
impl WallBreakEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallBreakEvent{ x, y } }
}
impl Event for WallBreakEvent {}

struct WallStartedBreakingEvent {
    pub x: i32,
    pub y: i32
}
impl WallStartedBreakingEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallStartedBreakingEvent{ x, y } }
}
impl Event for WallStartedBreakingEvent {}

struct WallStoppedBreakingEvent {
    pub x: i32,
    pub y: i32
}
impl WallStoppedBreakingEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallStoppedBreakingEvent{ x, y } }
}
impl Event for WallStoppedBreakingEvent {}

/*struct WallType {
    pub id,

}*/