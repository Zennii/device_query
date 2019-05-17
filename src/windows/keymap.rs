/// A list of supported keys that we can query from the OS. Outside of mod
/// keys, we only support English keys at the moment.
#[derive(Debug, PartialEq, Clone)]
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
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

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
    Capital,
    Delete,
    Insert,
    Tab,

    // Numberpad keys
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    // Math keys
    Add,
    Decimal,
    Divide,
    Multiply,
    Subtract,
}

impl Keycode {
    pub fn keycode_to_key(keycode: i32) -> Option<Keycode> {
        use winapi::winuser;

        // Matches keycode against utility/control keys, defaulting to `None`
        // if no match is found
        let mut key = match keycode {
            // Function keys
            winuser::VK_F1 => Some(Keycode::F1),
            winuser::VK_F2 => Some(Keycode::F2),
            winuser::VK_F3 => Some(Keycode::F3),
            winuser::VK_F4 => Some(Keycode::F4),
            winuser::VK_F5 => Some(Keycode::F5),
            winuser::VK_F6 => Some(Keycode::F6),
            winuser::VK_F7 => Some(Keycode::F7),
            winuser::VK_F8 => Some(Keycode::F8),
            winuser::VK_F9 => Some(Keycode::F9),
            winuser::VK_F10 => Some(Keycode::F10),
            winuser::VK_F11 => Some(Keycode::F11),
            winuser::VK_F12 => Some(Keycode::F12),
            winuser::VK_F13 => Some(Keycode::F13),
            winuser::VK_F14 => Some(Keycode::F14),
            winuser::VK_F15 => Some(Keycode::F15),
            winuser::VK_F16 => Some(Keycode::F16),
            winuser::VK_F17 => Some(Keycode::F17),
            winuser::VK_F18 => Some(Keycode::F18),
            winuser::VK_F19 => Some(Keycode::F19),
            winuser::VK_F20 => Some(Keycode::F20),
            winuser::VK_F21 => Some(Keycode::F21),
            winuser::VK_F22 => Some(Keycode::F22),
            winuser::VK_F23 => Some(Keycode::F23),
            winuser::VK_F24 => Some(Keycode::F24),

            // Miscellaneous control keys
            winuser::VK_SPACE => Some(Keycode::Space),
            winuser::VK_LCONTROL => Some(Keycode::LControl),
            winuser::VK_RCONTROL => Some(Keycode::RControl),
            winuser::VK_LSHIFT => Some(Keycode::LShift),
            winuser::VK_RSHIFT => Some(Keycode::RShift),
            winuser::VK_LMENU => Some(Keycode::LAlt),
            winuser::VK_RMENU => Some(Keycode::RAlt),
            winuser::VK_RETURN => Some(Keycode::Enter),
            winuser::VK_ESCAPE => Some(Keycode::Escape),
            winuser::VK_CAPITAL => Some(Keycode::Capital),
            winuser::VK_DELETE => Some(Keycode::Delete),
            winuser::VK_INSERT => Some(Keycode::Insert),
            winuser::VK_TAB => Some(Keycode::Tab),

            // Numberpad keys
            winuser::VK_NUMLOCK => Some(Keycode::Numlock),
            winuser::VK_NUMPAD0 => Some(Keycode::Numpad0),
            winuser::VK_NUMPAD1 => Some(Keycode::Numpad1),
            winuser::VK_NUMPAD2 => Some(Keycode::Numpad2),
            winuser::VK_NUMPAD3 => Some(Keycode::Numpad3),
            winuser::VK_NUMPAD4 => Some(Keycode::Numpad4),
            winuser::VK_NUMPAD5 => Some(Keycode::Numpad5),
            winuser::VK_NUMPAD6 => Some(Keycode::Numpad6),
            winuser::VK_NUMPAD7 => Some(Keycode::Numpad7),
            winuser::VK_NUMPAD8 => Some(Keycode::Numpad8),
            winuser::VK_NUMPAD9 => Some(Keycode::Numpad9),

            // Math keys
            winuser::VK_ADD => Some(Keycode::Add),
            winuser::VK_DECIMAL => Some(Keycode::Decimal),
            winuser::VK_DIVIDE => Some(Keycode::Divide),
            winuser::VK_MULTIPLY => Some(Keycode::Multiply),
            winuser::VK_SUBTRACT => Some(Keycode::Subtract),

            // Return `None` if no match is found
            _ => None,
        };

        // If the keycode is not matched against any control keys,
        // then check the char value of the code against alphanumeric keys
        if key.is_none() {
            let keycode = keycode as u8;
            key = match keycode as char {
                // Numeric keys
                '0' => Some(Keycode::Key0),
                '1' => Some(Keycode::Key1),
                '2' => Some(Keycode::Key2),
                '3' => Some(Keycode::Key3),
                '4' => Some(Keycode::Key4),
                '5' => Some(Keycode::Key5),
                '6' => Some(Keycode::Key6),
                '7' => Some(Keycode::Key7),
                '8' => Some(Keycode::Key8),
                '9' => Some(Keycode::Key9),

                // Letter keys
                'A' => Some(Keycode::A),
                'B' => Some(Keycode::B),
                'C' => Some(Keycode::C),
                'D' => Some(Keycode::D),
                'E' => Some(Keycode::E),
                'F' => Some(Keycode::F),
                'G' => Some(Keycode::G),
                'H' => Some(Keycode::H),
                'I' => Some(Keycode::I),
                'J' => Some(Keycode::J),
                'K' => Some(Keycode::K),
                'L' => Some(Keycode::L),
                'M' => Some(Keycode::M),
                'N' => Some(Keycode::N),
                'O' => Some(Keycode::O),
                'P' => Some(Keycode::P),
                'Q' => Some(Keycode::Q),
                'R' => Some(Keycode::R),
                'S' => Some(Keycode::S),
                'T' => Some(Keycode::T),
                'U' => Some(Keycode::U),
                'V' => Some(Keycode::V),
                'W' => Some(Keycode::W),
                'X' => Some(Keycode::X),
                'Y' => Some(Keycode::Y),
                'Z' => Some(Keycode::Z),

                // Return `None` if no match is found
                _ => None,
            }
        }

        key
    }
}
