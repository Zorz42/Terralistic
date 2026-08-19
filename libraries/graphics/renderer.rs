use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use arboard::Clipboard;

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::draw_list::{DrawCommand, DrawList, DrawTarget};
use crate::libraries::graphics::shadow::ShadowContext;
use crate::libraries::graphics::wgpu_backend::WgpuBackend;
use crate::libraries::graphics::window::Window;
use crate::libraries::graphics::Font;
use crate::libraries::timing;
use crate::libraries::ui;
use crate::libraries::ui::UiContext;

/// How many unread events to keep before the oldest start falling off. A frame produces a
/// handful, so this is only ever reached by a loop that presents without reading its input.
const MAX_QUEUED_EVENTS: usize = 1024;

/// The window, the input, and the frame being recorded.
///
/// Drawing does not happen here: a `render` call appends to `draw_list`, and `update_window`
/// hands the list to `backend`, the only part of the toolkit that knows wgpu exists.
pub struct GraphicsContext {
    backend: WgpuBackend,
    window: Window,
    /// Behind a `RefCell` because drawing takes `&self`: calls shaped like
    /// `graphics.font.render_text(graphics, ..)` borrow the context twice.
    draw_list: RefCell<DrawList>,
    events_queue: VecDeque<gfx::Event>,
    /// Whether the window has *ever* been pumped, never cleared. `update_window` collects input
    /// at the end of every frame, so the only pump `get_event` owes is the one before the first.
    window_pumped: bool,
    /// Draw into a logical-sized offscreen rather than a device-sized one. Only the golden
    /// harness asks: its images have to be the same whatever the DPI of the machine.
    render_at_logical_resolution: bool,
    window_open: bool,
    /// Which keys are held, maintained from the events as they go past - a UI element asks "is
    /// shift down" far more often than it reacts to shift.
    key_states: HashSet<gfx::Key>,
    /// `pub(crate)` because `RenderRect` draws its own shadow and lives in `libraries::ui`.
    pub(crate) shadow_context: ShadowContext,
    /// `None` where the system offers no clipboard: copy and paste stop working and nothing
    /// else does, so it is not a reason to refuse to open the window.
    clipboard_context: Option<Clipboard>,
    pub block_key_states: bool,
    pub scale: f32,
    real_scale: f32,
    scale_animation_timer: timing::FixedStep,
    frame_limiter: timing::FrameLimiter,
    prev_frame_time: std::time::Instant,
    pub font: Font,
    pub font_mono: Option<Font>,
}

impl GraphicsContext {
    /// Opens the window and brings up the renderer. Usually fails because the system does not
    /// support graphics.
    pub fn new(window_width: u32, window_height: u32, window_title: &str, font: &[u8], font_mono: Option<&[u8]>) -> Result<Self> {
        Self::new_with_visibility(window_width, window_height, window_title, font, font_mono, true)
    }

    /// Same as `new`, but the window is never mapped and the frame is drawn at logical
    /// resolution. Rendering goes to an offscreen texture, so a hidden window drives the whole
    /// renderer - only `present` needs a visible one. The golden-image tests use this.
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
            key_states: HashSet::new(),
            shadow_context,
            events_queue: VecDeque::new(),
            window_pumped: false,
            render_at_logical_resolution: !visible,
            window_open: true,
            clipboard_context: Clipboard::new().map_err(|error| println!("No clipboard available, copy and paste will do nothing: {error}")).ok(),
            block_key_states: false,
            scale: 1.0,
            real_scale: 1.0,
            scale_animation_timer: timing::FixedStep::for_animation(10),
            frame_limiter: timing::FrameLimiter::default(),
            prev_frame_time: std::time::Instant::now(),
            font,
            font_mono,
        };

        result.handle_window_resize();

        Ok(result)
    }

    /// Reallocates everything sized in window pixels, on a resize and once at startup.
    ///
    /// The frame is drawn at the display's **real** resolution. Layout is logical either way -
    /// the transform onto clip space is a ratio - but the real resolution lets an animation
    /// move one device pixel at a time and makes the blit a copy rather than an upscale. This
    /// is the one place the offscreen size is chosen.
    ///
    /// The clip space transform is recomputed here too, and must be: it is otherwise only set
    /// at the end of `update_window`, so the first frame drew through the identity - the whole
    /// window collapsed into the top left two-by-two of clip space - and the frame after a
    /// resize through the previous size. Both last one frame, which is why neither was noticed.
    fn handle_window_resize(&mut self) {
        let surface_size = self.window.drawable_size();
        let offscreen_size = if self.render_at_logical_resolution { self.window.size() } else { surface_size };
        self.backend.resize(offscreen_size, surface_size);

        let window_size = self.get_window_size();
        self.backend.update_normalization_transform(window_size);
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
        self.backend.clear_next_frame();
        self.draw_list.borrow_mut().clear();
    }

    /// Executes what the case recorded and reads the frame back. The flush is here rather than
    /// in `update_window` because a captured case never presents.
    #[cfg(feature = "render-tests")]
    pub fn capture_frame(&mut self) -> Result<gfx::Surface> {
        self.flush_draw_list();
        self.backend.read_pixels()
    }

    /// Jumps the wall-clock-driven scale and blur fades to their targets, so a golden does not
    /// depend on how long the test took.
    #[cfg(feature = "render-tests")]
    pub const fn settle_animations(&mut self) {
        self.real_scale = self.scale;
        self.backend.settle_blur();
    }

    /// Pumps the window system and turns what it reports into queued events and context state.
    fn poll_window(&mut self) {
        self.window_pumped = true;
        let poll = self.window.poll();

        if poll.resized {
            self.handle_window_resize();
        }
        if poll.closed {
            self.close_window();
        }
        // The release goes to whichever window took the focus, so anything held would stay
        // held forever - this is why alt-tabbing mid-stride does not leave the player walking.
        if poll.focus_lost {
            self.key_states.clear();
        }

        for event in poll.events {
            match event {
                gfx::Event::KeyPress(key, ..) => {
                    self.key_states.insert(key);
                }
                gfx::Event::KeyRelease(key, ..) => {
                    self.key_states.remove(&key);
                }
                _ => {}
            }
            self.events_queue.push_back(event);
        }

        // The oldest is the right end to drop: a caller that has ignored a thousand events
        // wants the recent ones if it ever looks.
        while self.events_queue.len() > MAX_QUEUED_EVENTS {
            self.events_queue.pop_front();
        }
    }

    /// The next event, or `None` once this frame has none left. Drains the queue
    /// `update_window` filled; only pumps the window itself before the very first frame.
    pub fn get_event(&mut self) -> Option<gfx::Event> {
        if self.events_queue.is_empty() && !self.window_pumped {
            self.poll_window();
        }
        self.events_queue.pop_front()
    }

    /// False once the user has closed the window, or the program has.
    #[must_use]
    pub const fn is_window_open(&self) -> bool {
        self.window_open
    }

    pub fn close_window(&mut self) {
        self.window_open = false;
        self.window.hide();
    }

    /// Ends the frame: executes what was recorded, presents it, collects the next frame's input.
    pub fn update_window(&mut self) {
        // The flush comes first, before the animations advance and the transform is recomputed.
        // That is what makes deferral invisible: a command executes with exactly the transform
        // and blur intensity it would have had immediately. Don't reorder it.
        self.flush_draw_list();

        self.backend.update_blur();

        while self.scale_animation_timer.step() {
            self.real_scale = ui::approach(self.real_scale, self.scale, 10.0, 0.001);
        }

        let window_size = self.get_window_size();
        self.backend.update_normalization_transform(window_size);
        if let Err(error) = self.backend.present() {
            println!("Error presenting frame: {error}");
        }

        self.limit_framerate();

        // Collecting input at the frame boundary is about *where the waiting happens*: the
        // events are the same ones either way, but on macOS pumping is what services the
        // layer's pending drawable, so the wait for the display lands here and can be most of
        // a frame. Inside the client's first 10ms it would spend the chunk-meshing budget.
        self.poll_window();
    }

    /// Sleeps for whatever is left of this frame's share of the wall clock - see `FrameLimiter`.
    fn limit_framerate(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.prev_frame_time);
        self.prev_frame_time = now;

        let owed = self.frame_limiter.owed_ms(elapsed.as_secs_f64() * 1000.0);
        if owed > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(owed / 1000.0));
        }
    }

    pub fn set_min_window_size(&mut self, size: gfx::FloatSize) {
        self.window.set_min_size(size);
    }

    /// Records a blur of whatever has already been drawn inside `rect`.
    pub(crate) fn blur_rect(&self, rect: gfx::Rect, radius: i32) {
        self.push_draw_command(DrawCommand::Blur { rect, radius });
    }

    pub const fn enable_blur(&mut self, enable: bool) {
        self.backend.set_blur_enabled(enable);
    }

    pub fn set_fps_limit(&mut self, fps: f32) {
        self.frame_limiter.set_fps_limit(fps);
    }

    pub fn disable_fps_limit(&mut self) {
        self.set_fps_limit(0.0);
    }

    pub fn enable_vsync(&mut self, enable: bool) {
        self.backend.set_vsync(enable);
    }
}

/// The window, pointer, keyboard and clipboard, which is everything a UI element is allowed to
/// observe. Kept deliberately separate from rendering - see `ui::UiContext`.
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
        !self.block_key_states && self.key_states.contains(&key)
    }

    fn get_clipboard_text(&mut self) -> Option<String> {
        self.clipboard_context.as_mut()?.get_text().ok()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        let Some(clipboard) = self.clipboard_context.as_mut() else { return };
        if let Err(error) = clipboard.set_text(text.to_owned()) {
            println!("Error setting clipboard contents: {error}");
        }
    }

    fn as_graphics_context(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

/// Where the game's drawing ends up. The commands sit here until `update_window` replays them
/// through the backend.
impl DrawTarget for GraphicsContext {
    fn push_draw_command(&self, command: DrawCommand) {
        self.draw_list.borrow_mut().push(command);
    }

    fn get_draw_area(&self) -> gfx::FloatSize {
        self.get_window_size()
    }
}
