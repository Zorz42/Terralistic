//! The window and the input: everything the toolkit needs from the operating system, and the
//! only module that knows winit exists. Above it `GraphicsContext` deals in `gfx::Event` and
//! `gfx::IntSize`; beside it `WgpuBackend` sees only something to hang a surface off.
//!
//! **The event loop is pumped, not run.** winit's `run_app` wants to own the process and call
//! back; the game's three main loops drive their own simulation and rendering, and inverting
//! them was not worth it. `pump_app_events` dispatches what is queued and returns. It costs a
//! repaint *during* a resize drag on macOS, and is desktop only - which is all this builds for.
//!
//! **Keys are physical positions, not labels.** `translate_key` maps winit's `KeyCode`, which
//! names a position on a US layout, so WASD stays a square on AZERTY. Typing is unaffected:
//! text arrives separately as `Event::TextInput`.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::window::{WindowAttributes, WindowId};

use crate::libraries::graphics as gfx;

/// A scroll wheel notch, in pixels. Wheels report lines and touchpads pixels; everything
/// downstream of `Event::MouseScroll` is written in notches, so pixels convert through this.
const PIXELS_PER_SCROLL_NOTCH: f32 = 16.0;

/// What one round of pumping produced. A snapshot rather than more `gfx::Event`s: a resize or
/// a close is not something a UI element can handle, so they never reach the event queue.
pub(super) struct Poll {
    pub events: Vec<gfx::Event>,
    /// The window changed size, or moved to a display with a different scale factor.
    pub resized: bool,
    /// The user asked for the window to go away.
    pub closed: bool,
    /// The window lost keyboard focus, so whatever was held down is not held down any more.
    pub focus_lost: bool,
}

/// The window, plus the event loop that feeds it.
pub(super) struct Window {
    event_loop: EventLoop<()>,
    state: State,
}

impl Window {
    /// Opens a window and returns once the window system has produced it. winit only hands out
    /// an `ActiveEventLoop` - the one thing that creates a window - from inside a callback, so
    /// this pumps until `resumed` has run.
    pub(super) fn new(title: &str, size: gfx::IntSize, visible: bool) -> Result<Self> {
        let event_loop = EventLoop::new()?;
        // The game's loop decides when the next frame is, so winit never waits.
        event_loop.set_control_flow(ControlFlow::Poll);

        let attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(LogicalSize::new(size.0, size.1))
            .with_resizable(true)
            .with_visible(visible);

        let mut result = Self {
            event_loop,
            state: State {
                attributes,
                window: None,
                creation_error: None,
                events: Vec::new(),
                mouse_pos: gfx::FloatPos(0.0, 0.0),
                geometry: Geometry::FALLBACK,
                resized: false,
                closed: false,
                focus_lost: false,
            },
        };

        // One pump is enough everywhere this builds, but the number of round trips is not part
        // of winit's contract, so this waits on the outcome rather than a fixed count.
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while result.state.window.is_none() && result.state.creation_error.is_none() {
            if std::time::Instant::now() > deadline {
                return Err(anyhow!("the window system never produced a window"));
            }
            result.event_loop.pump_app_events(Some(Duration::from_millis(1)), &mut result.state);
        }

        if let Some(error) = result.state.creation_error.take() {
            return Err(anyhow!(error));
        }
        Ok(result)
    }

    /// The window, for the backend to create its surface from. An `Arc` rather than a borrow
    /// so the surface can be `Surface<'static>` without the unsafe raw-handle constructor.
    pub(super) fn handle(&self) -> Result<Arc<winit::window::Window>> {
        self.state.window.clone().ok_or_else(|| anyhow!("the window is gone"))
    }

    /// Dispatches everything the window system has queued and returns what came of it.
    pub(super) fn poll(&mut self) -> Poll {
        self.event_loop.pump_app_events(Some(Duration::ZERO), &mut self.state);
        self.state.take_poll()
    }

    /// The window in logical pixels, which is what the game lays out and draws in.
    pub(super) const fn size(&self) -> gfx::IntSize {
        self.state.geometry.logical_size
    }

    /// The window in real device pixels, which is what the surface has to be configured at.
    /// On a `HiDPI` display this is a multiple of `size`.
    pub(super) const fn drawable_size(&self) -> gfx::IntSize {
        self.state.geometry.physical_size
    }

    /// Where the pointer is, in the same logical pixels as `size`.
    pub(super) const fn mouse_pos(&self) -> gfx::FloatPos {
        self.state.mouse_pos
    }

    /// Takes the window off the screen without dropping it, so an app that still has shutting
    /// down to do - saving a world - looks closed while it finishes rather than frozen.
    pub(super) fn hide(&self) {
        if let Some(window) = &self.state.window {
            window.set_visible(false);
        }
    }

    pub(super) fn set_min_size(&self, size: gfx::FloatSize) {
        if let Some(window) = &self.state.window {
            window.set_min_inner_size(Some(LogicalSize::new(size.0, size.1)));
        }
    }
}

/// How big the window is, cached - **for performance, not tidiness**. Layout asks ~540 times a
/// frame (every `Container`, the camera bounds, every chunk testing visibility), and winit does
/// not cache: `inner_size` and `scale_factor` are objc message sends on macOS at ~12us, which
/// measured at 40% of the game's wall clock and starved the chunk-meshing budget. The resize
/// events are the authority, so nothing is ever read back.
#[derive(Clone, Copy)]
struct Geometry {
    /// Real device pixels. What the surface is configured at.
    physical_size: gfx::IntSize,
    /// Physical divided by the scale factor. What the game draws in.
    logical_size: gfx::IntSize,
    scale_factor: f64,
}

impl Geometry {
    /// A window that has gone away reports 1x1, not 0x0: these sizes end up as divisors.
    const FALLBACK: Self = Self {
        physical_size: gfx::IntSize(1, 1),
        logical_size: gfx::IntSize(1, 1),
        scale_factor: 1.0,
    };

    fn new(physical: PhysicalSize<u32>, scale_factor: f64) -> Self {
        let physical = PhysicalSize::new(physical.width.max(1), physical.height.max(1));
        let logical = physical.to_logical::<f64>(scale_factor);
        Self {
            physical_size: gfx::IntSize(physical.width, physical.height),
            logical_size: gfx::IntSize((logical.width as u32).max(1), (logical.height as u32).max(1)),
            scale_factor,
        }
    }

    fn of(window: &winit::window::Window) -> Self {
        Self::new(window.inner_size(), window.scale_factor())
    }
}

/// The `ApplicationHandler` half: winit calls into this, and everything it learns is left
/// here for the next `poll` to collect.
struct State {
    attributes: WindowAttributes,
    window: Option<Arc<winit::window::Window>>,
    creation_error: Option<winit::error::OsError>,
    events: Vec<gfx::Event>,
    mouse_pos: gfx::FloatPos,
    /// Kept up to date from the resize events rather than read back - see `Geometry`.
    geometry: Geometry,
    resized: bool,
    closed: bool,
    focus_lost: bool,
}

impl State {
    fn take_poll(&mut self) -> Poll {
        Poll {
            events: std::mem::take(&mut self.events),
            resized: std::mem::take(&mut self.resized),
            closed: std::mem::take(&mut self.closed),
            focus_lost: std::mem::take(&mut self.focus_lost),
        }
    }

    fn handle_keyboard(&mut self, event: &winit::event::KeyEvent) {
        if let PhysicalKey::Code(code) = event.physical_key {
            if let Some(key) = translate_key(code) {
                self.events.push(match event.state {
                    ElementState::Pressed => gfx::Event::KeyPress(key, event.repeat),
                    ElementState::Released => gfx::Event::KeyRelease(key, event.repeat),
                });
            }
        }

        // Which characters a press produces is a separate question from which key it was:
        // shift, dead keys and the layout sit between. Control characters are dropped - a text
        // field wants the *text*, and backspace and friends already arrived above as keys.
        if event.state.is_pressed() {
            if let Some(text) = &event.text {
                let text: String = text.chars().filter(|c| !c.is_control()).collect();
                if !text.is_empty() {
                    self.events.push(gfx::Event::TextInput(text));
                }
            }
        }
    }

    fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        let notches = match delta {
            MouseScrollDelta::LineDelta(_, y) => y,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y as f32 / PIXELS_PER_SCROLL_NOTCH,
        };
        if notches != 0.0 {
            self.events.push(gfx::Event::MouseScroll(notches));
        }
    }
}

impl ApplicationHandler for State {
    /// Where the window is created. Called once per resume, hence the guard: the game has
    /// exactly one window for its whole life.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        match event_loop.create_window(self.attributes.clone()) {
            Ok(window) => {
                // Nothing else asks for focus, and macOS does not raise a window for an app it
                // does not consider active - which is what a pumped event loop leaves it.
                window.focus_window();
                self.geometry = Geometry::of(&window);
                self.window = Some(Arc::new(window));
            }
            Err(error) => self.creation_error = Some(error),
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => self.closed = true,
            WindowEvent::Resized(size) => {
                self.geometry = Geometry::new(size, self.geometry.scale_factor);
                self.resized = true;
            }
            // A scale change is a resize to the surface even at an unchanged logical size: the
            // drawable moved. The new physical size is not in the event, so this is the one
            // place that reads back - on a move between displays, not every frame.
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let physical = self
                    .window
                    .as_ref()
                    .map_or_else(|| PhysicalSize::new(self.geometry.physical_size.0, self.geometry.physical_size.1), |window| window.inner_size());
                self.geometry = Geometry::new(physical, scale_factor);
                self.resized = true;
            }
            // Only ever set, and cleared by the `poll` that collects it. Assigning `!focused`
            // loses a loss followed by a regain in the same pump - a fast alt-tab, which
            // leaves exactly the keys stuck down that clearing them prevents.
            WindowEvent::Focused(focused) => self.focus_lost |= !focused,
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical::<f64>(self.geometry.scale_factor);
                self.mouse_pos = gfx::FloatPos(position.x as f32, position.y as f32);
            }
            // `is_synthetic` marks the events replayed to describe the keyboard at a focus
            // change: not presses anyone made, and taking them types every key that is down.
            WindowEvent::KeyboardInput { event, is_synthetic: false, .. } => self.handle_keyboard(&event),
            WindowEvent::MouseWheel { delta, .. } => self.handle_mouse_wheel(delta),
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(key) = translate_mouse_button(button) {
                    self.events.push(match state {
                        ElementState::Pressed => gfx::Event::KeyPress(key, false),
                        ElementState::Released => gfx::Event::KeyRelease(key, false),
                    });
                }
            }
            _ => {}
        }
    }
}

/// Maps a physical key position onto the toolkit's key enum. `None` for everything the game
/// has no name for, which is most of the keyboard.
const fn translate_key(code: KeyCode) -> Option<gfx::Key> {
    use gfx::Key;
    Some(match code {
        KeyCode::Space => Key::Space,
        KeyCode::KeyA => Key::A,
        KeyCode::KeyB => Key::B,
        KeyCode::KeyC => Key::C,
        KeyCode::KeyD => Key::D,
        KeyCode::KeyE => Key::E,
        KeyCode::KeyF => Key::F,
        KeyCode::KeyG => Key::G,
        KeyCode::KeyH => Key::H,
        KeyCode::KeyI => Key::I,
        KeyCode::KeyJ => Key::J,
        KeyCode::KeyK => Key::K,
        KeyCode::KeyL => Key::L,
        KeyCode::KeyM => Key::M,
        KeyCode::KeyN => Key::N,
        KeyCode::KeyO => Key::O,
        KeyCode::KeyP => Key::P,
        KeyCode::KeyQ => Key::Q,
        KeyCode::KeyR => Key::R,
        KeyCode::KeyS => Key::S,
        KeyCode::KeyT => Key::T,
        KeyCode::KeyU => Key::U,
        KeyCode::KeyV => Key::V,
        KeyCode::KeyW => Key::W,
        KeyCode::KeyX => Key::X,
        KeyCode::KeyY => Key::Y,
        KeyCode::KeyZ => Key::Z,
        // Row and keypad both count: the hotbar is bound to these.
        KeyCode::Digit0 | KeyCode::Numpad0 => Key::Num0,
        KeyCode::Digit1 | KeyCode::Numpad1 => Key::Num1,
        KeyCode::Digit2 | KeyCode::Numpad2 => Key::Num2,
        KeyCode::Digit3 | KeyCode::Numpad3 => Key::Num3,
        KeyCode::Digit4 | KeyCode::Numpad4 => Key::Num4,
        KeyCode::Digit5 | KeyCode::Numpad5 => Key::Num5,
        KeyCode::Digit6 | KeyCode::Numpad6 => Key::Num6,
        KeyCode::Digit7 | KeyCode::Numpad7 => Key::Num7,
        KeyCode::Digit8 | KeyCode::Numpad8 => Key::Num8,
        KeyCode::Digit9 | KeyCode::Numpad9 => Key::Num9,
        KeyCode::Escape => Key::Escape,
        KeyCode::Enter | KeyCode::NumpadEnter => Key::Enter,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Insert => Key::Insert,
        KeyCode::Delete => Key::Delete,
        KeyCode::ArrowRight => Key::Right,
        KeyCode::ArrowLeft => Key::Left,
        KeyCode::ArrowDown => Key::Down,
        KeyCode::ArrowUp => Key::Up,
        KeyCode::F1 => Key::F1,
        KeyCode::F2 => Key::F2,
        KeyCode::F3 => Key::F3,
        KeyCode::F4 => Key::F4,
        KeyCode::F5 => Key::F5,
        KeyCode::F6 => Key::F6,
        KeyCode::F7 => Key::F7,
        KeyCode::F8 => Key::F8,
        KeyCode::F9 => Key::F9,
        KeyCode::F10 => Key::F10,
        KeyCode::F11 => Key::F11,
        KeyCode::F12 => Key::F12,
        KeyCode::ShiftLeft => Key::LeftShift,
        KeyCode::ControlLeft => Key::LeftControl,
        KeyCode::AltLeft => Key::LeftAlt,
        KeyCode::SuperLeft => Key::LeftSuper,
        KeyCode::ShiftRight => Key::RightShift,
        KeyCode::ControlRight => Key::RightControl,
        KeyCode::AltRight => Key::RightAlt,
        KeyCode::SuperRight => Key::RightSuper,
        _ => return None,
    })
}

/// Maps a mouse button onto the toolkit's key enum. The extra buttons are unbound.
const fn translate_mouse_button(button: MouseButton) -> Option<gfx::Key> {
    Some(match button {
        MouseButton::Left => gfx::Key::MouseLeft,
        MouseButton::Right => gfx::Key::MouseRight,
        MouseButton::Middle => gfx::Key::MouseMiddle,
        _ => return None,
    })
}
