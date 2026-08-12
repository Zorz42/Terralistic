use crate::libraries::graphics as gfx;

/// Everything a UI element needs in order to lay itself out and react to input.
///
/// This is deliberately the *whole* non-rendering surface of `GraphicsContext`: window size,
/// pointer, keyboard and clipboard. `get_container` and `on_event_inner` take a
/// `&dyn UiContext` rather than a `&GraphicsContext`, which is what makes layout and event
/// handling testable without a window or an OpenGL context. Drawing still needs the real
/// thing, so `render_inner` and `update_inner` keep taking `&mut GraphicsContext`.
///
/// If you are tempted to add a method here, check first that it is not a rendering
/// operation in disguise - the value of this trait is entirely in what it leaves out.
pub trait UiContext {
    /// Size of the window in logical (scaled) pixels.
    fn get_window_size(&self) -> gfx::FloatSize;
    /// Mouse position in logical (scaled) pixels, relative to the window's top left.
    fn get_mouse_pos(&self) -> gfx::FloatPos;
    /// Whether a key is currently held down.
    fn get_key_state(&self, key: gfx::Key) -> bool;
    /// Current clipboard contents, or `None` if the clipboard is empty or unavailable.
    fn get_clipboard_text(&mut self) -> Option<String>;
    /// Replaces the clipboard contents. Failures are reported and swallowed, since a
    /// missing clipboard should never take the game down.
    fn set_clipboard_text(&mut self, text: &str);

    /// The full rendering context, if this context has one.
    ///
    /// This is the escape hatch, and it exists because three menus do genuinely
    /// graphical work from an event handler: `world_creation` and `multiplayer_selector`
    /// build the next menu (whose labels are uploaded as textures) and `settings_menu`
    /// applies vsync, scale and the fps limit. Those branches are skipped under a headless
    /// context and therefore are not covered by tests.
    ///
    /// Nothing in the toolkit itself calls this. If you reach for it in a new widget, the
    /// work almost certainly belongs in `render_inner` or `update_inner` instead.
    fn as_graphics_context(&mut self) -> Option<&mut gfx::GraphicsContext> {
        None
    }
}

/// A `UiContext` with no window, no OpenGL context and no real clipboard, for tests.
///
/// Every input a UI element can observe is a plain field you set directly, so a test reads
/// as "put the mouse here, hold this key, send this event, assert on the result".
#[cfg(test)]
pub struct HeadlessContext {
    window_size: gfx::FloatSize,
    mouse_pos: gfx::FloatPos,
    pressed_keys: std::collections::HashSet<gfx::Key>,
    clipboard: Option<String>,
}

#[cfg(test)]
impl HeadlessContext {
    /// A context with a 1000x800 window, the mouse parked at the origin and nothing held.
    #[must_use]
    pub fn new() -> Self {
        Self {
            window_size: gfx::FloatSize(1000.0, 800.0),
            mouse_pos: gfx::FloatPos(0.0, 0.0),
            pressed_keys: std::collections::HashSet::new(),
            clipboard: None,
        }
    }

    pub const fn set_window_size(&mut self, size: gfx::FloatSize) {
        self.window_size = size;
    }

    pub const fn set_mouse_pos(&mut self, pos: gfx::FloatPos) {
        self.mouse_pos = pos;
    }

    /// Presses or releases a key, as `GraphicsContext` would when it sees the SDL event.
    pub fn set_key_state(&mut self, key: gfx::Key, pressed: bool) {
        if pressed {
            self.pressed_keys.insert(key);
        } else {
            self.pressed_keys.remove(&key);
        }
    }
}

#[cfg(test)]
impl UiContext for HeadlessContext {
    fn get_window_size(&self) -> gfx::FloatSize {
        self.window_size
    }

    fn get_mouse_pos(&self) -> gfx::FloatPos {
        self.mouse_pos
    }

    fn get_key_state(&self, key: gfx::Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    fn get_clipboard_text(&mut self) -> Option<String> {
        self.clipboard.clone()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        self.clipboard = Some(text.to_owned());
    }
}
