use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::swap;

use arboard::Clipboard;

use anyhow::{anyhow, Result};

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::draw_list::{DrawCommand, DrawList, DrawTarget};
use crate::libraries::graphics::events::sdl_event_to_gfx_event;
use crate::libraries::graphics::shadow::ShadowContext;
use crate::libraries::graphics::wgpu_backend::WgpuBackend;
use crate::libraries::graphics::Font;
use crate::libraries::graphics::UiContext;

/// The window, the input, and the frame currently being recorded.
///
/// Drawing does not happen here. A `render` call appends to `draw_list`, and `update_window`
/// hands the whole list to `backend`, which is the only part of the toolkit that knows about
/// OpenGL. Everything between those two points is plain data - see `gfx::draw_list`.
pub struct GraphicsContext {
    /// Declared before `sdl_window` on purpose. The backend holds a `wgpu::Surface` created
    /// from the window's raw handle, and fields drop in declaration order, so this is what
    /// keeps the surface from outliving the window it points at.
    backend: WgpuBackend,
    sdl_window: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump,
    /// The frame being recorded. Behind a `RefCell` because drawing takes `&self`: the
    /// toolkit is full of `graphics.font.render_text(graphics, ..)` shaped calls that borrow
    /// the context twice, which was fine when drawing went straight to OpenGL and has to
    /// stay fine now.
    draw_list: RefCell<DrawList>,
    events_queue: VecDeque<gfx::Event>,
    window_open: bool,
    // Keep track of all Key states as a hashmap
    key_states: HashMap<gfx::Key, bool>,
    events: Vec<gfx::Event>,
    pub(super) shadow_context: ShadowContext,
    clipboard_context: Clipboard,
    pub block_key_states: bool,
    pub scale: f32,
    real_scale: f32,
    scale_animation_timer: gfx::AnimationTimer,
    min_ms_per_frame: f32,
    frames_so_far: u32,
    ms_so_far: f32,
    prev_frame_time: std::time::Instant,
    pub font: Font,
    pub font_mono: Option<Font>,
}

impl GraphicsContext {
    /// Initializes all the values needed for rendering.
    /// It usually fails because the system doesn't support graphics.
    pub fn new(window_width: u32, window_height: u32, window_title: &str, font: &[u8], font_mono: Option<&[u8]>) -> Result<Self> {
        Self::new_with_visibility(window_width, window_height, window_title, font, font_mono, true)
    }

    /// Same as `new`, but the window is never mapped on screen.
    ///
    /// Rendering goes to an offscreen texture rather than to the surface, so a hidden window
    /// is enough to drive the whole renderer - only `present` needs a visible one. This is
    /// what the golden-image tests use, so running them does not flash windows across the
    /// desktop.
    #[cfg(feature = "render-tests")]
    pub fn new_hidden(window_width: u32, window_height: u32, font: &[u8], font_mono: Option<&[u8]>) -> Result<Self> {
        Self::new_with_visibility(window_width, window_height, "Terralistic render tests", font, font_mono, false)
    }

    fn new_with_visibility(window_width: u32, window_height: u32, window_title: &str, font: &[u8], font_mono: Option<&[u8]>, visible: bool) -> Result<Self> {
        let sdl = sdl2::init();
        let sdl = sdl.map_err(|e| anyhow!(e))?;
        let video_subsystem = sdl.video();
        let video_subsystem = video_subsystem.map_err(|e| anyhow!(e))?;

        let mut window_builder = video_subsystem.window(window_title, window_width, window_height);
        window_builder.position_centered().resizable();
        // wgpu reaches the window through its raw handle, and on macOS that handle has to be
        // a Metal view. Without this SDL panics when the handle is asked for.
        #[cfg(target_os = "macos")]
        window_builder.metal_view();
        if !visible {
            window_builder.hidden();
        }
        let sdl_window = window_builder.build()?;

        let backend = WgpuBackend::new(&sdl_window, window_size_of(&sdl_window), drawable_size_of(&sdl_window))?;
        // Uploads a texture, so it has to come after the device exists.
        let shadow_context = ShadowContext::new();

        let font = Font::new(font, false)?;
        let font_mono = if let Some(data) = font_mono { Some(Font::new(data, true)?) } else { None };

        let mut result = Self {
            backend,
            sdl_window,
            sdl_event_pump: sdl.event_pump().map_err(|e| anyhow!(e))?,
            draw_list: RefCell::new(DrawList::new()),
            key_states: HashMap::new(),
            events: Vec::new(),
            shadow_context,
            events_queue: VecDeque::new(),
            window_open: true,
            clipboard_context: Clipboard::new()?,
            block_key_states: false,
            scale: 1.0,
            real_scale: 1.0,
            scale_animation_timer: gfx::AnimationTimer::new(10),
            min_ms_per_frame: 0.0,
            frames_so_far: 0,
            ms_so_far: 0.0,
            prev_frame_time: std::time::Instant::now(),
            font,
            font_mono,
        };

        result.handle_window_resize();

        Ok(result)
    }

    /// Is called every time the window is resized.
    pub fn handle_window_resize(&mut self) {
        self.backend.resize(window_size_of(&self.sdl_window), drawable_size_of(&self.sdl_window));
    }

    /// Hands the recorded frame to the backend and starts a new one.
    fn flush_draw_list(&mut self) {
        let window_size = self.get_window_size();
        self.backend.execute(&self.draw_list.borrow(), window_size);
        self.draw_list.borrow_mut().clear();
    }

    /// Prepares a deterministic offscreen frame for the golden-image tests.
    #[cfg(feature = "render-tests")]
    pub fn begin_capture_frame(&mut self) {
        let window_size = self.get_window_size();
        self.backend.update_normalization_transform(window_size);
        self.backend.clear_next_frame();
        self.draw_list.borrow_mut().clear();
    }

    /// Executes whatever the case recorded, then reads the frame back into a `Surface`.
    ///
    /// The flush has to happen here rather than in `update_window`, because a captured case
    /// never presents - there is nothing to show on a hidden window.
    #[cfg(feature = "render-tests")]
    pub fn capture_frame(&mut self) -> Result<gfx::Surface> {
        self.flush_draw_list();
        self.backend.read_pixels()
    }

    /// Jumps the scale and blur animations straight to their target values.
    ///
    /// Both are driven by wall-clock timers, so without this a golden would depend on how
    /// long the test happened to take.
    #[cfg(feature = "render-tests")]
    pub const fn settle_animations(&mut self) {
        self.real_scale = self.scale;
        self.backend.settle_blur();
    }

    /// Returns an array of events, such as key presses.
    fn get_events(&mut self) -> Vec<gfx::Event> {
        let mut sdl_events = vec![];

        for sdl_event in self.sdl_event_pump.poll_iter() {
            sdl_events.push(sdl_event);
        }

        for sdl_event in sdl_events {
            match sdl_event {
                // handle window resize
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(_width, _height),
                    ..
                } => {
                    self.handle_window_resize();
                }
                // handle quit event
                sdl2::event::Event::Quit { .. } => {
                    self.close_window();
                }
                _ => {}
            }

            if let Some(event) = sdl_event_to_gfx_event(&sdl_event) {
                // if event is a key press event update the key states to true
                if let gfx::Event::KeyPress(key, ..) = event {
                    self.set_key_state(key, true);
                }
                // if event is a key release event update the key states to false
                if let gfx::Event::KeyRelease(key, ..) = event {
                    self.set_key_state(key, false);
                }

                self.events.push(event);
            }
        }

        let mut result = Vec::new();
        swap(&mut result, &mut self.events);

        result
    }

    /// Returns an event, returns None if there are no events
    pub fn get_event(&mut self) -> Option<gfx::Event> {
        for event in self.get_events() {
            self.events_queue.push_back(event);
        }
        self.events_queue.pop_front()
    }

    /// Checks if the window is open, this becomes false, when the user closes the window, or the program closes it
    #[must_use]
    pub const fn is_window_open(&self) -> bool {
        self.window_open
    }

    /// Closes the window
    pub const fn close_window(&mut self) {
        self.window_open = false;
    }

    /// Should be called after rendering
    pub fn update_window(&mut self) {
        // The flush comes first, before the animations advance and before the transform is
        // recomputed. That is what makes deferring the frame's drawing to here invisible: a
        // command recorded during this frame is executed with exactly the transform and the
        // blur intensity it would have been drawn with immediately.
        self.flush_draw_list();

        self.backend.update_blur();

        while self.scale_animation_timer.frame_ready() {
            self.real_scale += (self.scale - self.real_scale) / 10.0;
            if f32::abs(self.real_scale - self.scale) < 0.001 {
                self.real_scale = self.scale;
            }
        }

        let window_size = self.get_window_size();
        self.backend.update_normalization_transform(window_size);
        if let Err(error) = self.backend.present() {
            println!("Error presenting frame: {error}");
        }

        self.frames_so_far += 1;
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.prev_frame_time).as_millis() as f32;
        self.ms_so_far += delta;
        self.prev_frame_time = now;
        if self.ms_so_far < self.min_ms_per_frame * self.frames_so_far as f32 {
            std::thread::sleep(std::time::Duration::from_millis((self.min_ms_per_frame * self.frames_so_far as f32 - self.ms_so_far) as u64));
        }
    }

    /// Sets the minimum window size
    pub fn set_min_window_size(&mut self, size: gfx::FloatSize) -> Result<()> {
        self.sdl_window.set_minimum_size(size.0 as u32, size.1 as u32).map_err(|e| anyhow!(e))
    }

    /// Sets key state
    fn set_key_state(&mut self, key: gfx::Key, state: bool) {
        *self.key_states.entry(key).or_insert(false) = state;
    }

    /// Records a blur of whatever has already been drawn inside `rect`.
    pub(super) fn blur_rect(&self, rect: gfx::Rect, radius: i32) {
        self.push_draw_command(DrawCommand::Blur { rect, radius });
    }

    pub const fn enable_blur(&mut self, enable: bool) {
        self.backend.set_blur_enabled(enable);
    }

    pub fn set_fps_limit(&mut self, fps: f32) {
        self.min_ms_per_frame = 1000.0 / fps;
        self.frames_so_far = 0;
        self.ms_so_far = 0.0;
    }

    pub const fn disable_fps_limit(&mut self) {
        self.min_ms_per_frame = 0.0;
    }

    pub fn enable_vsync(&mut self, enable: bool) {
        self.backend.set_vsync(enable);
    }
}

/// The window, pointer, keyboard and clipboard, which is everything a UI element is allowed
/// to observe. Kept deliberately separate from rendering - see `gfx::UiContext`.
impl UiContext for GraphicsContext {
    fn get_window_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(self.sdl_window.size().0 as f32 / self.real_scale, self.sdl_window.size().1 as f32 / self.real_scale)
    }

    fn get_mouse_pos(&self) -> gfx::FloatPos {
        gfx::FloatPos(
            self.sdl_event_pump.mouse_state().x() as f32 / self.real_scale,
            self.sdl_event_pump.mouse_state().y() as f32 / self.real_scale,
        )
    }

    fn get_key_state(&self, key: gfx::Key) -> bool {
        !self.block_key_states && *self.key_states.get(&key).unwrap_or(&false)
    }

    fn get_clipboard_text(&mut self) -> Option<String> {
        self.clipboard_context.get_text().ok()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        if let Err(error) = self.clipboard_context.set_text(text.to_owned()) {
            println!("Error setting clipboard contents: {error}");
        }
    }

    fn as_graphics_context(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

/// The window in logical pixels, which is what the game draws in.
fn window_size_of(window: &sdl2::video::Window) -> gfx::IntSize {
    let size = window.size();
    gfx::IntSize(size.0, size.1)
}

/// The window in real device pixels, which is what the surface has to be configured at.
///
/// On a `HiDPI` display these differ by the backing scale factor. The OpenGL backend
/// hardcoded that ratio as 2.0, which was wrong on every other kind of display; asking SDL
/// keeps the game rendering at logical resolution while the final upscale matches whatever
/// the screen actually is.
fn drawable_size_of(window: &sdl2::video::Window) -> gfx::IntSize {
    let drawable = window.drawable_size();
    if drawable.0 == 0 || drawable.1 == 0 {
        return window_size_of(window);
    }
    gfx::IntSize(drawable.0, drawable.1)
}

/// Where the game's drawing ends up. The commands sit here until `update_window` replays
/// them through the backend.
impl DrawTarget for GraphicsContext {
    fn push_draw_command(&self, command: DrawCommand) {
        self.draw_list.borrow_mut().push(command);
    }

    fn get_draw_area(&self) -> gfx::FloatSize {
        self.get_window_size()
    }
}
