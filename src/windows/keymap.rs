/// A list of supported keys. Outside of mod keys, only English keys are supported.
#[derive(Debug, PartialEq, Clone)]
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

impl KeyCode {
    pub fn keycode_to_key(keycode: i32) -> Option<KeyCode> {
        use winapi::um::winuser;

        // Matches KeyCode against utility/control keys, defaulting to `None`
        // if no match is found
        let mut key = match keycode {
            // Function keys
            winuser::VK_F1 => Some(KeyCode::F1),
            winuser::VK_F2 => Some(KeyCode::F2),
            winuser::VK_F3 => Some(KeyCode::F3),
            winuser::VK_F4 => Some(KeyCode::F4),
            winuser::VK_F5 => Some(KeyCode::F5),
            winuser::VK_F6 => Some(KeyCode::F6),
            winuser::VK_F7 => Some(KeyCode::F7),
            winuser::VK_F8 => Some(KeyCode::F8),
            winuser::VK_F9 => Some(KeyCode::F9),
            winuser::VK_F10 => Some(KeyCode::F10),
            winuser::VK_F11 => Some(KeyCode::F11),
            winuser::VK_F12 => Some(KeyCode::F12),
            winuser::VK_F13 => Some(KeyCode::F13),
            winuser::VK_F14 => Some(KeyCode::F14),
            winuser::VK_F15 => Some(KeyCode::F15),
            winuser::VK_F16 => Some(KeyCode::F16),
            winuser::VK_F17 => Some(KeyCode::F17),
            winuser::VK_F18 => Some(KeyCode::F18),
            winuser::VK_F19 => Some(KeyCode::F19),
            winuser::VK_F20 => Some(KeyCode::F20),
            winuser::VK_F21 => Some(KeyCode::F21),
            winuser::VK_F22 => Some(KeyCode::F22),
            winuser::VK_F23 => Some(KeyCode::F23),
            winuser::VK_F24 => Some(KeyCode::F24),

            // Miscellaneous control keys
            winuser::VK_SPACE => Some(KeyCode::Space),
            winuser::VK_LCONTROL => Some(KeyCode::LControl),
            winuser::VK_RCONTROL => Some(KeyCode::RControl),
            winuser::VK_LSHIFT => Some(KeyCode::LShift),
            winuser::VK_RSHIFT => Some(KeyCode::RShift),
            winuser::VK_LMENU => Some(KeyCode::LAlt),
            winuser::VK_RMENU => Some(KeyCode::RAlt),
            winuser::VK_RETURN => Some(KeyCode::Enter),
            winuser::VK_ESCAPE => Some(KeyCode::Escape),
            winuser::VK_CAPITAL => Some(KeyCode::Capital),
            winuser::VK_DELETE => Some(KeyCode::Delete),
            winuser::VK_INSERT => Some(KeyCode::Insert),
            winuser::VK_TAB => Some(KeyCode::Tab),

            // Numberpad keys
            winuser::VK_NUMLOCK => Some(KeyCode::Numlock),
            winuser::VK_NUMPAD0 => Some(KeyCode::Numpad0),
            winuser::VK_NUMPAD1 => Some(KeyCode::Numpad1),
            winuser::VK_NUMPAD2 => Some(KeyCode::Numpad2),
            winuser::VK_NUMPAD3 => Some(KeyCode::Numpad3),
            winuser::VK_NUMPAD4 => Some(KeyCode::Numpad4),
            winuser::VK_NUMPAD5 => Some(KeyCode::Numpad5),
            winuser::VK_NUMPAD6 => Some(KeyCode::Numpad6),
            winuser::VK_NUMPAD7 => Some(KeyCode::Numpad7),
            winuser::VK_NUMPAD8 => Some(KeyCode::Numpad8),
            winuser::VK_NUMPAD9 => Some(KeyCode::Numpad9),

            // Math keys
            winuser::VK_ADD => Some(KeyCode::Add),
            winuser::VK_DECIMAL => Some(KeyCode::Decimal),
            winuser::VK_DIVIDE => Some(KeyCode::Divide),
            winuser::VK_MULTIPLY => Some(KeyCode::Multiply),
            winuser::VK_SUBTRACT => Some(KeyCode::Subtract),

            // Return `None` if no match is found
            _ => None,
        };

        // If the KeyCode is not matched against any control keys,
        // then check the char value of the code against alphanumeric keys
        if key.is_none() {
            let keycode = keycode as u8;
            key = match keycode as char {
                // Numeric keys
                '0' => Some(KeyCode::Key0),
                '1' => Some(KeyCode::Key1),
                '2' => Some(KeyCode::Key2),
                '3' => Some(KeyCode::Key3),
                '4' => Some(KeyCode::Key4),
                '5' => Some(KeyCode::Key5),
                '6' => Some(KeyCode::Key6),
                '7' => Some(KeyCode::Key7),
                '8' => Some(KeyCode::Key8),
                '9' => Some(KeyCode::Key9),

                // Letter keys
                'A' => Some(KeyCode::A),
                'B' => Some(KeyCode::B),
                'C' => Some(KeyCode::C),
                'D' => Some(KeyCode::D),
                'E' => Some(KeyCode::E),
                'F' => Some(KeyCode::F),
                'G' => Some(KeyCode::G),
                'H' => Some(KeyCode::H),
                'I' => Some(KeyCode::I),
                'J' => Some(KeyCode::J),
                'K' => Some(KeyCode::K),
                'L' => Some(KeyCode::L),
                'M' => Some(KeyCode::M),
                'N' => Some(KeyCode::N),
                'O' => Some(KeyCode::O),
                'P' => Some(KeyCode::P),
                'Q' => Some(KeyCode::Q),
                'R' => Some(KeyCode::R),
                'S' => Some(KeyCode::S),
                'T' => Some(KeyCode::T),
                'U' => Some(KeyCode::U),
                'V' => Some(KeyCode::V),
                'W' => Some(KeyCode::W),
                'X' => Some(KeyCode::X),
                'Y' => Some(KeyCode::Y),
                'Z' => Some(KeyCode::Z),

                // Return `None` if no match is found
                _ => None,
            }
        }

        key
    }
}
