pub(crate) trait ServerModule {
    fn init(&mut self);
    fn update(&mut self, delta_time: f32);
    fn stop(&mut self);
}