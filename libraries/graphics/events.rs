/// A collection of all supported events
#[derive(Clone)]
pub enum Event {
    // key press and release deprecated_events
    KeyPress(Key, bool),
    KeyRelease(Key, bool),
    MouseScroll(f32),
    TextInput(String),
}

/// Translates sdl type events to our event type
pub fn sdl_event_to_gfx_event(sdl_event: &sdl2::event::Event) -> Option<Event> {
    match sdl_event {
        sdl2::event::Event::KeyDown {
            keycode: Some(keycode),
            repeat,
            ..
        } => sdl_key_to_gfx_key(*keycode).map(|key| Event::KeyPress(key, *repeat)),

        sdl2::event::Event::KeyUp {
            keycode: Some(keycode),
            repeat,
            ..
        } => sdl_key_to_gfx_key(*keycode).map(|key| Event::KeyRelease(key, *repeat)),

        sdl2::event::Event::MouseWheel { y, .. } => Some(Event::MouseScroll(*y as f32)),
        sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
            sdl_mouse_button_to_gfx_key(*mouse_btn).map(|key| Event::KeyPress(key, false))
        }
        sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
            sdl_mouse_button_to_gfx_key(*mouse_btn).map(|key| Event::KeyRelease(key, false))
        }
        sdl2::event::Event::TextInput { text, .. } => Some(Event::TextInput(text.clone())),

        _ => None,
    }
}

/// A collection of all keys on the keyboard and mouse.
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

/// This function converts a sdl key to a graphics key
const fn sdl_key_to_gfx_key(key: sdl2::keyboard::Keycode) -> Option<Key> {
    match key {
        sdl2::keyboard::Keycode::Space => Some(Key::Space),
        sdl2::keyboard::Keycode::A => Some(Key::A),
        sdl2::keyboard::Keycode::B => Some(Key::B),
        sdl2::keyboard::Keycode::C => Some(Key::C),
        sdl2::keyboard::Keycode::D => Some(Key::D),
        sdl2::keyboard::Keycode::E => Some(Key::E),
        sdl2::keyboard::Keycode::F => Some(Key::F),
        sdl2::keyboard::Keycode::G => Some(Key::G),
        sdl2::keyboard::Keycode::H => Some(Key::H),
        sdl2::keyboard::Keycode::I => Some(Key::I),
        sdl2::keyboard::Keycode::J => Some(Key::J),
        sdl2::keyboard::Keycode::K => Some(Key::K),
        sdl2::keyboard::Keycode::L => Some(Key::L),
        sdl2::keyboard::Keycode::M => Some(Key::M),
        sdl2::keyboard::Keycode::N => Some(Key::N),
        sdl2::keyboard::Keycode::O => Some(Key::O),
        sdl2::keyboard::Keycode::P => Some(Key::P),
        sdl2::keyboard::Keycode::Q => Some(Key::Q),
        sdl2::keyboard::Keycode::R => Some(Key::R),
        sdl2::keyboard::Keycode::S => Some(Key::S),
        sdl2::keyboard::Keycode::T => Some(Key::T),
        sdl2::keyboard::Keycode::U => Some(Key::U),
        sdl2::keyboard::Keycode::V => Some(Key::V),
        sdl2::keyboard::Keycode::W => Some(Key::W),
        sdl2::keyboard::Keycode::X => Some(Key::X),
        sdl2::keyboard::Keycode::Y => Some(Key::Y),
        sdl2::keyboard::Keycode::Z => Some(Key::Z),
        sdl2::keyboard::Keycode::Escape => Some(Key::Escape),
        sdl2::keyboard::Keycode::Return => Some(Key::Enter),
        sdl2::keyboard::Keycode::Tab => Some(Key::Tab),
        sdl2::keyboard::Keycode::Backspace => Some(Key::Backspace),
        sdl2::keyboard::Keycode::Insert => Some(Key::Insert),
        sdl2::keyboard::Keycode::Delete => Some(Key::Delete),
        sdl2::keyboard::Keycode::Right => Some(Key::Right),
        sdl2::keyboard::Keycode::Left => Some(Key::Left),
        sdl2::keyboard::Keycode::Down => Some(Key::Down),
        sdl2::keyboard::Keycode::Up => Some(Key::Up),
        sdl2::keyboard::Keycode::F1 => Some(Key::F1),
        sdl2::keyboard::Keycode::F2 => Some(Key::F2),
        sdl2::keyboard::Keycode::F3 => Some(Key::F3),
        sdl2::keyboard::Keycode::F4 => Some(Key::F4),
        sdl2::keyboard::Keycode::F5 => Some(Key::F5),
        sdl2::keyboard::Keycode::F6 => Some(Key::F6),
        sdl2::keyboard::Keycode::F7 => Some(Key::F7),
        sdl2::keyboard::Keycode::F8 => Some(Key::F8),
        sdl2::keyboard::Keycode::F9 => Some(Key::F9),
        sdl2::keyboard::Keycode::F10 => Some(Key::F10),
        sdl2::keyboard::Keycode::F11 => Some(Key::F11),
        sdl2::keyboard::Keycode::F12 => Some(Key::F12),
        sdl2::keyboard::Keycode::LShift => Some(Key::LeftShift),
        sdl2::keyboard::Keycode::LCtrl => Some(Key::LeftControl),
        sdl2::keyboard::Keycode::LAlt => Some(Key::LeftAlt),
        sdl2::keyboard::Keycode::LGui => Some(Key::LeftSuper),
        sdl2::keyboard::Keycode::RShift => Some(Key::RightShift),
        sdl2::keyboard::Keycode::RCtrl => Some(Key::RightControl),
        sdl2::keyboard::Keycode::RAlt => Some(Key::RightAlt),
        sdl2::keyboard::Keycode::RGui => Some(Key::RightSuper),
        _ => None,
    }
}

/// This function converts a sdl mouse button to a graphics key
pub const fn sdl_mouse_button_to_gfx_key(button: sdl2::mouse::MouseButton) -> Option<Key> {
    match button {
        sdl2::mouse::MouseButton::Left => Some(Key::MouseLeft),
        sdl2::mouse::MouseButton::Right => Some(Key::MouseRight),
        sdl2::mouse::MouseButton::Middle => Some(Key::MouseMiddle),
        _ => None,
    }
}
