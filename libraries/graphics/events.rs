//! What the user did, as data.
//!
//! Nothing here knows where an event came from. The window system's own types are translated
//! in `gfx::window`, which is the only module that has ever heard of one, and the headless
//! half of the test suite constructs these values directly.

/// A collection of all supported events
#[derive(Clone)]
pub enum Event {
    /// A key went down. The flag is set when the platform is auto-repeating a held key
    /// rather than reporting a fresh press.
    KeyPress(Key, bool),
    /// A key came back up. The flag mirrors `KeyPress`.
    KeyRelease(Key, bool),
    /// Scroll movement, in pixels. Positive is away from the user. A trackpad reports pixels
    /// natively, so this tracks the finger; a wheel detent is worth `PIXELS_PER_SCROLL_LINE`.
    MouseScroll(f32),
    /// Text the user typed, already resolved through the keyboard layout. This is what a
    /// text field consumes; `KeyPress` is what a key binding consumes.
    TextInput(String),
}

/// A collection of all keys on the keyboard and mouse.
///
/// A letter here names a *position* on the keyboard rather than a label, so `W` is whichever
/// key sits where W sits on a US layout - see `gfx::window` for why.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    MouseLeft,
    MouseRight,
    MouseMiddle,
}
