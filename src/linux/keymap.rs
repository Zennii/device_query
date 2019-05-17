#[derive(Debug, PartialEq, Clone)]
/// A list of supported keys. Outside of mod keys, only English keys are supported.
pub enum KeyCode {
    // Numeric keys
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Letter keys
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

    // Function keys
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

    // Miscellaneous control keys
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Enter,
}

impl KeyCode {
    pub fn keysym_to_key(keysym: u32) -> Option<KeyCode> {
        use x11::keysym;

        // Match the KeyCode against keys, returning the key
        // if found or `None` if no match is found
        match keysym {
            // Numeric keys
            keysym::XK_0 => Some(KeyCode::Key0),
            keysym::XK_1 => Some(KeyCode::Key1),
            keysym::XK_2 => Some(KeyCode::Key2),
            keysym::XK_3 => Some(KeyCode::Key3),
            keysym::XK_4 => Some(KeyCode::Key4),
            keysym::XK_5 => Some(KeyCode::Key5),
            keysym::XK_6 => Some(KeyCode::Key6),
            keysym::XK_7 => Some(KeyCode::Key7),
            keysym::XK_8 => Some(KeyCode::Key8),
            keysym::XK_9 => Some(KeyCode::Key9),

            // Letter keys
            keysym::XK_A => Some(KeyCode::A),
            keysym::XK_B => Some(KeyCode::B),
            keysym::XK_C => Some(KeyCode::C),
            keysym::XK_D => Some(KeyCode::D),
            keysym::XK_E => Some(KeyCode::E),
            keysym::XK_F => Some(KeyCode::F),
            keysym::XK_G => Some(KeyCode::G),
            keysym::XK_H => Some(KeyCode::H),
            keysym::XK_I => Some(KeyCode::I),
            keysym::XK_J => Some(KeyCode::J),
            keysym::XK_K => Some(KeyCode::K),
            keysym::XK_L => Some(KeyCode::L),
            keysym::XK_M => Some(KeyCode::M),
            keysym::XK_N => Some(KeyCode::N),
            keysym::XK_O => Some(KeyCode::O),
            keysym::XK_P => Some(KeyCode::P),
            keysym::XK_Q => Some(KeyCode::Q),
            keysym::XK_R => Some(KeyCode::R),
            keysym::XK_S => Some(KeyCode::S),
            keysym::XK_T => Some(KeyCode::T),
            keysym::XK_U => Some(KeyCode::U),
            keysym::XK_V => Some(KeyCode::V),
            keysym::XK_W => Some(KeyCode::W),
            keysym::XK_X => Some(KeyCode::X),
            keysym::XK_Y => Some(KeyCode::Y),
            keysym::XK_Z => Some(KeyCode::Z),

            // Function keys
            keysym::XK_F1 => Some(KeyCode::F1),
            keysym::XK_F2 => Some(KeyCode::F2),
            keysym::XK_F3 => Some(KeyCode::F3),
            keysym::XK_F4 => Some(KeyCode::F4),
            keysym::XK_F5 => Some(KeyCode::F5),
            keysym::XK_F6 => Some(KeyCode::F6),
            keysym::XK_F7 => Some(KeyCode::F7),
            keysym::XK_F8 => Some(KeyCode::F8),
            keysym::XK_F9 => Some(KeyCode::F9),
            keysym::XK_F10 => Some(KeyCode::F10),
            keysym::XK_F11 => Some(KeyCode::F11),
            keysym::XK_F12 => Some(KeyCode::F12),

            // Miscellaneous control keys
            keysym::XK_Escape => Some(KeyCode::Escape),
            keysym::XK_space => Some(KeyCode::Space),
            keysym::XK_Control_L => Some(KeyCode::LControl),
            keysym::XK_Control_R => Some(KeyCode::RControl),
            keysym::XK_Shift_L => Some(KeyCode::LShift),
            keysym::XK_Shift_R => Some(KeyCode::RShift),
            keysym::XK_Alt_L => Some(KeyCode::LAlt),
            keysym::XK_Alt_R => Some(KeyCode::RAlt),
            keysym::XK_Return => Some(KeyCode::Enter),

            // Return `None` if no match is found
            _ => None,
        }
    }
}
