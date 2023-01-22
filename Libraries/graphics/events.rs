use sdl2;

/**
A collection of all supported deprecated_events
 */
#[derive(Clone)]
pub enum Event {
    // key press and release deprecated_events
    KeyPress(Key, bool),
    KeyRelease(Key, bool),
    MouseScroll(f64),
    TextInput(String),
}

/**
Translates sdl type deprecated_events to our event type
 */
pub(crate) fn sdl_event_to_gfx_event(sdl_event: sdl2::event::Event) -> Option<Event> {
    match sdl_event {
        sdl2::event::Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
            Some(Event::KeyPress(sdl_key_to_gfx_key(keycode), repeat))
        }

        sdl2::event::Event::KeyUp { keycode: Some(keycode), repeat, .. } => {
            Some(Event::KeyRelease(sdl_key_to_gfx_key(keycode), repeat))
        }

        sdl2::event::Event::MouseWheel { y, .. } => {
            Some(Event::MouseScroll(y as f64))
        }
        sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
            Some(Event::KeyPress(sdl_mouse_button_to_gfx_key(mouse_btn), false))
        }
        sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
            Some(Event::KeyRelease(sdl_mouse_button_to_gfx_key(mouse_btn), false))
        }
        sdl2::event::Event::TextInput { text, .. } => {
            Some(Event::TextInput(text.clone()))
        }

        _ => None,
    }
}

/**
A collection of all keys on the keyboard and mouse.
 */
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Unknown,
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

/**
This function converts a sdl key to a graphics key
 */
fn sdl_key_to_gfx_key(key: sdl2::keyboard::Keycode) -> Key {
    match key {
        sdl2::keyboard::Keycode::Space => Key::Space,
        sdl2::keyboard::Keycode::A => Key::A,
        sdl2::keyboard::Keycode::B => Key::B,
        sdl2::keyboard::Keycode::C => Key::C,
        sdl2::keyboard::Keycode::D => Key::D,
        sdl2::keyboard::Keycode::E => Key::E,
        sdl2::keyboard::Keycode::F => Key::F,
        sdl2::keyboard::Keycode::G => Key::G,
        sdl2::keyboard::Keycode::H => Key::H,
        sdl2::keyboard::Keycode::I => Key::I,
        sdl2::keyboard::Keycode::J => Key::J,
        sdl2::keyboard::Keycode::K => Key::K,
        sdl2::keyboard::Keycode::L => Key::L,
        sdl2::keyboard::Keycode::M => Key::M,
        sdl2::keyboard::Keycode::N => Key::N,
        sdl2::keyboard::Keycode::O => Key::O,
        sdl2::keyboard::Keycode::P => Key::P,
        sdl2::keyboard::Keycode::Q => Key::Q,
        sdl2::keyboard::Keycode::R => Key::R,
        sdl2::keyboard::Keycode::S => Key::S,
        sdl2::keyboard::Keycode::T => Key::T,
        sdl2::keyboard::Keycode::U => Key::U,
        sdl2::keyboard::Keycode::V => Key::V,
        sdl2::keyboard::Keycode::W => Key::W,
        sdl2::keyboard::Keycode::X => Key::X,
        sdl2::keyboard::Keycode::Y => Key::Y,
        sdl2::keyboard::Keycode::Z => Key::Z,
        sdl2::keyboard::Keycode::Escape => Key::Escape,
        sdl2::keyboard::Keycode::Return => Key::Enter,
        sdl2::keyboard::Keycode::Tab => Key::Tab,
        sdl2::keyboard::Keycode::Backspace => Key::Backspace,
        sdl2::keyboard::Keycode::Insert => Key::Insert,
        sdl2::keyboard::Keycode::Delete => Key::Delete,
        sdl2::keyboard::Keycode::Right => Key::Right,
        sdl2::keyboard::Keycode::Left => Key::Left,
        sdl2::keyboard::Keycode::Down => Key::Down,
        sdl2::keyboard::Keycode::Up => Key::Up,
        sdl2::keyboard::Keycode::F1 => Key::F1,
        sdl2::keyboard::Keycode::F2 => Key::F2,
        sdl2::keyboard::Keycode::F3 => Key::F3,
        sdl2::keyboard::Keycode::F4 => Key::F4,
        sdl2::keyboard::Keycode::F5 => Key::F5,
        sdl2::keyboard::Keycode::F6 => Key::F6,
        sdl2::keyboard::Keycode::F7 => Key::F7,
        sdl2::keyboard::Keycode::F8 => Key::F8,
        sdl2::keyboard::Keycode::F9 => Key::F9,
        sdl2::keyboard::Keycode::F10 => Key::F10,
        sdl2::keyboard::Keycode::F11 => Key::F11,
        sdl2::keyboard::Keycode::F12 => Key::F12,
        sdl2::keyboard::Keycode::LShift => Key::LeftShift,
        sdl2::keyboard::Keycode::LCtrl => Key::LeftControl,
        sdl2::keyboard::Keycode::LAlt => Key::LeftAlt,
        sdl2::keyboard::Keycode::LGui => Key::LeftSuper,
        sdl2::keyboard::Keycode::RShift => Key::RightShift,
        sdl2::keyboard::Keycode::RCtrl => Key::RightControl,
        sdl2::keyboard::Keycode::RAlt => Key::RightAlt,
        sdl2::keyboard::Keycode::RGui => Key::RightSuper,
        _ => Key::Unknown,
    }
}

/**
This function converts a sdl mouse button to a graphics key
 */
pub fn sdl_mouse_button_to_gfx_key(button: sdl2::mouse::MouseButton) -> Key {
    match button {
        sdl2::mouse::MouseButton::Left => Key::MouseLeft,
        sdl2::mouse::MouseButton::Right => Key::MouseRight,
        sdl2::mouse::MouseButton::Middle => Key::MouseMiddle,
        _ => Key::Unknown,
    }
}