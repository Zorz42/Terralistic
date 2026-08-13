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
use crate::libraries::graphics::UiContext;

/// How many unread events to keep before the oldest start falling off. A frame produces a
/// handful, so this is only ever reached by a loop that presents without reading its input.
const MAX_QUEUED_EVENTS: usize = 1024;

/// The window, the input, and the frame currently being recorded.
///
/// Drawing does not happen here. A `render` call appends to `draw_list`, and `update_window`
/// hands the whole list to `backend`, which is the only part of the toolkit that knows about
/// wgpu. Everything between those two points is plain data - see `gfx::draw_list`.
pub struct GraphicsContext {
    backend: WgpuBackend,
    window: Window,
    /// The frame being recorded. Behind a `RefCell` because drawing takes `&self`: the toolkit
    /// is full of `graphics.font.render_text(graphics, ..)` shaped calls that borrow the
    /// context twice.
    draw_list: RefCell<DrawList>,
    /// Events read off the window but not yet handed to the caller.
    events_queue: VecDeque<gfx::Event>,
    /// Whether the window has *ever* been pumped. It is never cleared, and that is the point:
    /// once `update_window` has run a frame it collects the input at the end of every one, so
    /// the only pump `get_event` ever has to do itself is the one before the first frame.
    window_pumped: bool,
    /// Draw into a logical-sized offscreen rather than a device-sized one. Only the
    /// golden-image harness asks for this: its images are committed at the window's logical
    /// size and have to be the same whatever the DPI of the machine running them.
    render_at_logical_resolution: bool,
    window_open: bool,
    /// Which keys are currently held, maintained from the press and release events as they go
    /// past - a UI element asks "is shift down" far more often than it reacts to shift.
    key_states: HashSet<gfx::Key>,
    pub(super) shadow_context: ShadowContext,
    /// `None` where the system has no clipboard to offer. Copy and paste stop working; nothing
    /// else does, which is why this is not a reason to refuse to open the window.
    clipboard_context: Option<Clipboard>,
    pub block_key_states: bool,
    pub scale: f32,
    real_scale: f32,
    scale_animation_timer: gfx::AnimationTimer,
    frame_limiter: FrameLimiter,
    prev_frame_time: std::time::Instant,
    pub font: Font,
    pub font_mono: Option<Font>,
}

/// How long the frames so far *should* have taken against how long they did, in milliseconds.
///
/// The limit is an **average** rather than a per-frame cap: the sleep at the end of a frame is
/// the difference between the two sides, so a frame that overran is made up by the next ones
/// instead of pushing the whole session behind.
///
/// Both sides only ever grow, which is why they are `f64`: as `f32` a 16ms increment stops
/// being representable after about four hours, the elapsed side stalls while the target side
/// climbs, and the sleep grows without bound.
#[derive(Default)]
pub(super) struct FrameLimiter {
    /// Zero means unlimited.
    min_ms_per_frame: f64,
    frames_so_far: u64,
    ms_so_far: f64,
}

impl FrameLimiter {
    /// Caps the frame rate at `fps`. A non-positive `fps` means no limit, rather than a
    /// division by zero and a sleep measured in centuries.
    ///
    /// The ledger is only cleared when the limit actually changes: it means nothing across a
    /// change of target, but re-setting the *same* limit has to be free, because the settings
    /// menu applies every setting on every event it sees. Clearing it each time would leave the
    /// limiter capping each frame on its own rather than averaging over them.
    pub(super) fn set_fps_limit(&mut self, fps: f32) {
        let min_ms_per_frame = if fps > 0.0 { 1000.0 / f64::from(fps) } else { 0.0 };
        if (min_ms_per_frame - self.min_ms_per_frame).abs() > f64::EPSILON {
            *self = Self {
                min_ms_per_frame,
                frames_so_far: 0,
                ms_so_far: 0.0,
            };
        }
    }

    /// Books a frame that took `elapsed_ms` and answers how long to sleep for.
    ///
    /// **The debt is capped at one frame.** Without that the ledger is repaid at full speed
    /// however long it took to run up, so anything that stops the loop for a while - a world
    /// loading, a laptop waking, a breakpoint - buys that many frames of completely uncapped
    /// rendering afterwards. A five minute pause at a 60 fps limit is eighteen thousand of them.
    /// Making up a frame or two of jitter is the point; making up a stall the player did not ask
    /// for is not.
    pub(super) fn owed_ms(&mut self, elapsed_ms: f64) -> f64 {
        if self.min_ms_per_frame <= 0.0 {
            return 0.0;
        }

        self.frames_so_far += 1;
        self.ms_so_far += elapsed_ms;

        let owed = self.min_ms_per_frame * self.frames_so_far as f64 - self.ms_so_far;
        if owed < -self.min_ms_per_frame {
            self.ms_so_far = self.min_ms_per_frame * (self.frames_so_far as f64 + 1.0);
        }
        f64::max(owed, 0.0)
    }
}

impl GraphicsContext {
    /// Opens the window and brings up the renderer. Usually fails because the system does not
    /// support graphics.
    pub fn new(window_width: u32, window_height: u32, window_title: &str, font: &[u8], font_mono: Option<&[u8]>) -> Result<Self> {
        Self::new_with_visibility(window_width, window_height, window_title, font, font_mono, true)
    }

    /// Same as `new`, but the window is never mapped on screen and the frame is drawn at the
    /// window's logical resolution.
    ///
    /// Rendering goes to an offscreen texture rather than to the surface, so a hidden window is
    /// enough to drive the whole renderer - only `present` needs a visible one. This is what
    /// the golden-image tests use, so running them does not flash windows across the desktop.
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
            scale_animation_timer: gfx::AnimationTimer::new(10),
            frame_limiter: FrameLimiter::default(),
            prev_frame_time: std::time::Instant::now(),
            font,
            font_mono,
        };

        result.handle_window_resize();

        Ok(result)
    }

    /// Reallocates everything sized in window pixels. Called when the window is resized, which
    /// `poll_window` notices, and once at startup.
    ///
    /// The frame is drawn at the display's **real** resolution, not the logical one. Layout is
    /// in logical pixels either way - the transform onto clip space is a ratio and does not
    /// care how many pixels the target has - but drawing at the real resolution lets a smooth
    /// animation move one device pixel at a time instead of jumping two, and makes the final
    /// blit a copy rather than an upscale. This is the one place the offscreen size is chosen.
    ///
    /// The clip space transform is recomputed here too, and has to be. It is otherwise only
    /// touched at the end of `update_window`, so the frame recorded before the *first* one drew
    /// through the identity - the whole window's worth of drawing landing in the top left
    /// two-by-two pixels of clip space - and the frame after a resize drew through the previous
    /// size. Both are one frame long, which is exactly why neither was ever noticed.
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

    /// Executes whatever the case recorded, then reads the frame back into a `Surface`. The
    /// flush happens here rather than in `update_window` because a captured case never
    /// presents - there is nothing to show on a hidden window.
    #[cfg(feature = "render-tests")]
    pub fn capture_frame(&mut self) -> Result<gfx::Surface> {
        self.flush_draw_list();
        self.backend.read_pixels()
    }

    /// Jumps the scale and blur fades straight to their targets, both being driven by the wall
    /// clock, so a golden does not depend on how long the test took.
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
        // Nothing reaches an unfocused window, so anything held at that moment would stay held
        // forever - the release arrives at whichever window took the focus. This is why
        // alt-tabbing mid-stride does not leave the player walking.
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

        // Dropping the oldest is the right end to drop: what a caller that has ignored a
        // thousand events wants, if it ever looks, is the recent ones.
        while self.events_queue.len() > MAX_QUEUED_EVENTS {
            self.events_queue.pop_front();
        }
    }

    /// The next event, or `None` once there are none left this frame.
    ///
    /// This just drains the queue `update_window` filled at the end of the previous frame. It
    /// only pumps the window itself before the very first one, when nothing has yet.
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

    pub const fn close_window(&mut self) {
        self.window_open = false;
    }

    /// Ends the frame: executes what was recorded, presents it, and collects the next frame's
    /// input.
    pub fn update_window(&mut self) {
        // The flush comes first, before the animations advance and before the transform is
        // recomputed. That is what makes deferring the frame's drawing to here invisible: a
        // command executes with exactly the transform and blur intensity it would have been
        // drawn with immediately. Don't reorder it.
        self.flush_draw_list();

        self.backend.update_blur();

        while self.scale_animation_timer.frame_ready() {
            self.real_scale = gfx::approach(self.real_scale, self.scale, 10.0, 0.001);
        }

        let window_size = self.get_window_size();
        self.backend.update_normalization_transform(window_size);
        if let Err(error) = self.backend.present() {
            println!("Error presenting frame: {error}");
        }

        self.limit_framerate();

        // Collecting input here, at the frame boundary, is about *where the waiting happens* -
        // the events are the same ones, delivered at the same point in the next frame either
        // way. On macOS pumping the event loop is what services the layer's pending drawable,
        // so with any slack in the frame the wait for the display lands in the pump, and it
        // can be most of a frame. `client/game/core_client.rs` gives `walls.rs` and `lights.rs`
        // the first 10ms of its loop to rebuild chunk meshes; a pump inside that window spends
        // the budget on waiting and the world takes minutes to finish drawing.
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
    pub(super) fn blur_rect(&self, rect: gfx::Rect, radius: i32) {
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
/// observe. Kept deliberately separate from rendering - see `gfx::UiContext`.
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
