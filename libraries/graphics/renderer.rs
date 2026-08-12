use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use arboard::Clipboard;

use anyhow::Result;

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::draw_list::{DrawCommand, DrawList, DrawTarget};
use crate::libraries::graphics::shadow::ShadowContext;
use crate::libraries::graphics::wgpu_backend::WgpuBackend;
use crate::libraries::graphics::window::Window;
use crate::libraries::graphics::Font;
use crate::libraries::graphics::UiContext;

/// The window, the input, and the frame currently being recorded.
///
/// Drawing does not happen here. A `render` call appends to `draw_list`, and `update_window`
/// hands the whole list to `backend`, which is the only part of the toolkit that knows about
/// wgpu. Everything between those two points is plain data - see `gfx::draw_list`.
pub struct GraphicsContext {
    backend: WgpuBackend,
    window: Window,
    /// The frame being recorded. Behind a `RefCell` because drawing takes `&self`: the
    /// toolkit is full of `graphics.font.render_text(graphics, ..)` shaped calls that borrow
    /// the context twice, which was fine when drawing went straight to OpenGL and has to
    /// stay fine now.
    draw_list: RefCell<DrawList>,
    /// Events that have been read off the window but not yet handed to the caller.
    events_queue: VecDeque<gfx::Event>,
    /// Whether the window has been pumped since the last `update_window`, so that draining
    /// the queue does not pump it again mid-frame.
    polled_this_frame: bool,
    window_open: bool,
    /// Which keys are currently held. Maintained from the press and release events as they
    /// go past, because a UI element asks "is shift down" far more often than it reacts to
    /// shift being pressed.
    key_states: HashMap<gfx::Key, bool>,
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
        let window = Window::new(window_title, gfx::IntSize(window_width, window_height), visible)?;

        let backend = WgpuBackend::new(&window.handle()?, window.size(), window.drawable_size())?;
        // Uploads a texture, so it has to come after the device exists.
        let shadow_context = ShadowContext::new();

        let font = Font::new(font, false)?;
        let font_mono = if let Some(data) = font_mono { Some(Font::new(data, true)?) } else { None };

        let mut result = Self {
            backend,
            window,
            draw_list: RefCell::new(DrawList::new()),
            key_states: HashMap::new(),
            shadow_context,
            events_queue: VecDeque::new(),
            polled_this_frame: false,
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

    /// Reallocates everything that is sized in window pixels. Called when the window is
    /// resized, which `get_event` notices, and once at startup.
    pub fn handle_window_resize(&mut self) {
        self.backend.resize(self.window.size(), self.window.drawable_size());
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

    /// Pumps the window system and turns what it reports into queued events and context state.
    fn poll_window(&mut self) {
        self.polled_this_frame = true;
        let poll = self.window.poll();

        if poll.resized {
            self.handle_window_resize();
        }
        if poll.closed {
            self.close_window();
        }
        // Nothing reaches a window that is not focused, so anything held down at that moment
        // would otherwise stay held forever - the release arrives at whichever window took
        // the focus. This is why alt-tabbing mid-stride does not leave the player walking.
        if poll.focus_lost {
            self.key_states.clear();
        }

        for event in poll.events {
            match event {
                gfx::Event::KeyPress(key, ..) => self.set_key_state(key, true),
                gfx::Event::KeyRelease(key, ..) => self.set_key_state(key, false),
                _ => {}
            }
            self.events_queue.push_back(event);
        }
    }

    /// Returns the next event, or `None` once there are none left this frame.
    ///
    /// This normally just drains the queue `update_window` filled at the end of the previous
    /// frame. It only pumps the window itself when nothing has yet this frame, which happens
    /// before the first frame and would happen for a caller that never presents.
    pub fn get_event(&mut self) -> Option<gfx::Event> {
        if self.events_queue.is_empty() && !self.polled_this_frame {
            self.poll_window();
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

        // Collect the next frame's input here, at the frame boundary, rather than letting
        // the first `get_event` of the next frame do it.
        //
        // This is about *where the waiting happens*, not about latency - the events are the
        // same ones, delivered at the same point in the next frame either way. On macOS
        // pumping the event loop is what services the layer's pending drawable, so with any
        // slack in the frame the pump is where the wait for the display lands, and it can be
        // most of a frame. `client/game/core_client.rs` starts a timer at the top of its loop
        // and gives `walls.rs` and `lights.rs` the first 10ms of it to rebuild chunk meshes;
        // a pump inside that window spends the budget on waiting and the world takes minutes
        // to finish drawing. Under SDL this wait sat in `present`, which is to say here.
        self.polled_this_frame = false;
        self.poll_window();
    }

    /// Sets the minimum window size
    pub fn set_min_window_size(&mut self, size: gfx::FloatSize) {
        self.window.set_min_size(size);
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
        let size = self.window.size();
        gfx::FloatSize(size.0 as f32 / self.real_scale, size.1 as f32 / self.real_scale)
    }

    fn get_mouse_pos(&self) -> gfx::FloatPos {
        let pos = self.window.mouse_pos();
        gfx::FloatPos(pos.0 / self.real_scale, pos.1 / self.real_scale)
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
