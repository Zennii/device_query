#[derive(Debug, PartialEq, Clone)]
/// A list of supported keys that we can query from the OS. Outside of mod
/// keys, we only support English keys at the moment.
pub enum Keycode {
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

impl Keycode {
    pub fn keysym_to_key(keysym: u32) -> Option<Keycode> {
        use x11::keysym;

        // Match the keycode against keys, returning the key
        // if found or `None` if no match is found
        match keysym {
            // Numeric keys
            keysym::XK_0 => Some(Keycode::Key0),
            keysym::XK_1 => Some(Keycode::Key1),
            keysym::XK_2 => Some(Keycode::Key2),
            keysym::XK_3 => Some(Keycode::Key3),
            keysym::XK_4 => Some(Keycode::Key4),
            keysym::XK_5 => Some(Keycode::Key5),
            keysym::XK_6 => Some(Keycode::Key6),
            keysym::XK_7 => Some(Keycode::Key7),
            keysym::XK_8 => Some(Keycode::Key8),
            keysym::XK_9 => Some(Keycode::Key9),

            // Letter keys
            keysym::XK_A => Some(Keycode::A),
            keysym::XK_B => Some(Keycode::B),
            keysym::XK_C => Some(Keycode::C),
            keysym::XK_D => Some(Keycode::D),
            keysym::XK_E => Some(Keycode::E),
            keysym::XK_F => Some(Keycode::F),
            keysym::XK_G => Some(Keycode::G),
            keysym::XK_H => Some(Keycode::H),
            keysym::XK_I => Some(Keycode::I),
            keysym::XK_J => Some(Keycode::J),
            keysym::XK_K => Some(Keycode::K),
            keysym::XK_L => Some(Keycode::L),
            keysym::XK_M => Some(Keycode::M),
            keysym::XK_N => Some(Keycode::N),
            keysym::XK_O => Some(Keycode::O),
            keysym::XK_P => Some(Keycode::P),
            keysym::XK_Q => Some(Keycode::Q),
            keysym::XK_R => Some(Keycode::R),
            keysym::XK_S => Some(Keycode::S),
            keysym::XK_T => Some(Keycode::T),
            keysym::XK_U => Some(Keycode::U),
            keysym::XK_V => Some(Keycode::V),
            keysym::XK_W => Some(Keycode::W),
            keysym::XK_X => Some(Keycode::X),
            keysym::XK_Y => Some(Keycode::Y),
            keysym::XK_Z => Some(Keycode::Z),

            // Function keys
            keysym::XK_F1 => Some(Keycode::F1),
            keysym::XK_F2 => Some(Keycode::F2),
            keysym::XK_F3 => Some(Keycode::F3),
            keysym::XK_F4 => Some(Keycode::F4),
            keysym::XK_F5 => Some(Keycode::F5),
            keysym::XK_F6 => Some(Keycode::F6),
            keysym::XK_F7 => Some(Keycode::F7),
            keysym::XK_F8 => Some(Keycode::F8),
            keysym::XK_F9 => Some(Keycode::F9),
            keysym::XK_F10 => Some(Keycode::F10),
            keysym::XK_F11 => Some(Keycode::F11),
            keysym::XK_F12 => Some(Keycode::F12),

            // Miscellaneous control keys
            keysym::XK_Escape => Some(Keycode::Escape),
            keysym::XK_space => Some(Keycode::Space),
            keysym::XK_Control_L => Some(Keycode::LControl),
            keysym::XK_Control_R => Some(Keycode::RControl),
            keysym::XK_Shift_L => Some(Keycode::LShift),
            keysym::XK_Shift_R => Some(Keycode::RShift),
            keysym::XK_Alt_L => Some(Keycode::LAlt),
            keysym::XK_Alt_R => Some(Keycode::RAlt),
            keysym::XK_Return => Some(Keycode::Enter),

            // Return `None` if no match is found
            _ => None,
        }
    }
}
