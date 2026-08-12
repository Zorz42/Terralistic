//! The window and the input, which is everything the toolkit needs from the operating system.
//!
//! This is the only module that knows winit exists. `GraphicsContext` above it deals in
//! `gfx::Event` and `gfx::IntSize`, and `WgpuBackend` beside it only ever sees the window as
//! something to hang a surface off.
//!
//! # Why the event loop is pumped rather than run
//!
//! winit wants to own the process: you hand `run_app` an `ApplicationHandler` and it calls
//! you back. The game is the opposite shape - `while graphics.is_window_open() { .. }` in
//! `client/game/core_client.rs`, `client/menus/title_screen_renderer.rs` and
//! `server/server_ui/ui_manager.rs`, all of which drive simulation and rendering themselves.
//! `EventLoopExtPumpEvents::pump_app_events` is the supported way to keep that shape: it
//! dispatches whatever the window system has queued and returns.
//!
//! What that costs, and why it is acceptable here:
//!
//! - **Rendering happens outside the loop.** winit warns that a platform which drives
//!   painting through a callback (macOS `drawRect`) can show artifacts while a window is
//!   being dragged to a new size. The game renders continuously anyway, so what it gives up
//!   is a repaint *during* the resize drag, not correctness afterwards.
//! - **`pump_app_events` is desktop only** - Windows, macOS, X11 and Wayland. Those are the
//!   only targets this game builds for.
//!
//! The alternative was inverting three main loops, an entity simulation and a server tick
//! around a callback. It was not worth it.
//!
//! # Keys are physical, not what the key is labelled
//!
//! `translate_key` maps winit's `KeyCode`, which names a *position* on a US layout, so
//! `Key::W` is whichever key sits where W sits on QWERTY. That is what a game wants: WASD
//! stays a square on AZERTY and on Dvorak. Typing is unaffected, because text arrives
//! separately as `Event::TextInput` with whatever the layout actually produced.
//!
//! SDL, which this replaced, reported layout-mapped keycodes instead, so on a non-US layout
//! the movement keys move.

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

/// A scroll wheel notch, in pixels.
///
/// Wheels report whole lines and touchpads report pixels, but everything downstream of
/// `Event::MouseScroll` - `Scrollable`, the server console, the module editor - is written
/// in notches. This is the conversion for the devices that report the other unit.
const PIXELS_PER_SCROLL_NOTCH: f32 = 16.0;

/// What one round of pumping the event loop produced.
///
/// Deliberately a snapshot rather than a stream of `gfx::Event`s: a resize and a close are
/// not things a UI element can handle, so they never reach the event queue.
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
    /// Opens a window and returns once the window system has actually produced it.
    ///
    /// winit only hands out an `ActiveEventLoop` - the one thing that can create a window -
    /// from inside a callback, so this pumps the loop until `resumed` has run.
    pub(super) fn new(title: &str, size: gfx::IntSize, visible: bool) -> Result<Self> {
        let event_loop = EventLoop::new()?;
        // Nothing here ever waits for an event: the game loop decides when the next frame
        // happens, and the pump below always uses a zero timeout.
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
                resized: false,
                closed: false,
                focus_lost: false,
            },
        };

        // One pump is enough on every platform this builds for, but the number of round
        // trips a backend needs is not part of winit's contract, so this waits on the
        // outcome rather than on a fixed count.
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

    /// The window itself, for the backend to create its surface from.
    ///
    /// An `Arc` rather than a borrow so that the surface can be `Surface<'static>` without
    /// the unsafe raw-handle constructor: it keeps the window alive for as long as wgpu
    /// needs it, whatever order the fields around it drop in.
    pub(super) fn handle(&self) -> Result<Arc<winit::window::Window>> {
        self.state.window.clone().ok_or_else(|| anyhow!("the window is gone"))
    }

    /// Dispatches everything the window system has queued and returns what came of it.
    pub(super) fn poll(&mut self) -> Poll {
        self.event_loop.pump_app_events(Some(Duration::ZERO), &mut self.state);
        self.state.take_poll()
    }

    /// The window in logical pixels, which is what the game lays out and draws in.
    pub(super) fn size(&self) -> gfx::IntSize {
        let (size, scale) = self.physical_size_and_scale();
        let logical = size.to_logical::<f64>(scale);
        gfx::IntSize(logical.width as u32, logical.height as u32)
    }

    /// The window in real device pixels, which is what the surface has to be configured at.
    ///
    /// On a `HiDPI` display this is a multiple of `size`. The OpenGL backend hardcoded that
    /// multiple as 2.0, which was wrong on every display that is not `HiDPI`.
    pub(super) fn drawable_size(&self) -> gfx::IntSize {
        let (size, _) = self.physical_size_and_scale();
        gfx::IntSize(size.width, size.height)
    }

    /// Where the pointer is, in the same logical pixels as `size`.
    pub(super) const fn mouse_pos(&self) -> gfx::FloatPos {
        self.state.mouse_pos
    }

    pub(super) fn set_min_size(&self, size: gfx::FloatSize) {
        if let Some(window) = &self.state.window {
            window.set_min_inner_size(Some(LogicalSize::new(size.0, size.1)));
        }
    }

    /// A window that has gone away reports 1x1 rather than 0x0: every size here ends up as a
    /// divisor somewhere downstream, and zero would take the layout arithmetic with it.
    fn physical_size_and_scale(&self) -> (PhysicalSize<u32>, f64) {
        self.state.window.as_ref().map_or((PhysicalSize::new(1, 1), 1.0), |window| {
            let size = window.inner_size();
            let size = PhysicalSize::new(size.width.max(1), size.height.max(1));
            (size, window.scale_factor())
        })
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

    fn scale_factor(&self) -> f64 {
        self.window.as_ref().map_or(1.0, |window| window.scale_factor())
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

        // The characters a keypress produces, which is a separate question from which key it
        // was: shift, dead keys and the layout all sit between the two. Control characters
        // are filtered out because a text field wants the *text*, and backspace, enter and
        // friends already arrived above as keys.
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
    /// Where the window is created. Called once on desktop, once per resume on mobile, hence
    /// the guard: the game has exactly one window for its whole life.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        match event_loop.create_window(self.attributes.clone()) {
            Ok(window) => {
                // A game window that opens behind whatever the player launched it from is
                // useless, and nothing else ever asks for the focus. macOS in particular
                // will not raise a window for an application it does not consider active,
                // which is exactly the state a pumped event loop leaves it in.
                window.focus_window();
                self.window = Some(Arc::new(window));
            }
            Err(error) => self.creation_error = Some(error),
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => self.closed = true,
            // A scale factor change is a resize as far as the surface is concerned, even
            // when the logical size is unchanged: the drawable size moved.
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => self.resized = true,
            WindowEvent::Focused(focused) => self.focus_lost = !focused,
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical::<f64>(self.scale_factor());
                self.mouse_pos = gfx::FloatPos(position.x as f32, position.y as f32);
            }
            // `is_synthetic` marks the events a platform replays to describe the keyboard
            // state at focus change. They are not presses the user made, and taking them
            // would type a character for every key that happened to be down.
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
        // The number row and the numeric keypad both count: the hotbar is bound to these,
        // and which one the player reaches for is up to them.
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
