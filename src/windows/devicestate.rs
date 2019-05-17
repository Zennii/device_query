use crate::{Keycode, MouseState};
use winapi::{shared::windef::POINT, um::winuser};

pub struct DeviceState;

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_pointer(&self) -> MouseState {
        let point: &mut POINT = &mut POINT { x: 0, y: 0 }; // Create a new, empty point

        // Create the mouse coordinate tuple
        let coordinates = if unsafe { winuser::GetCursorPos(point) != 0 } {
            let point = *point;
            (point.x, point.y)
        } else {
            (0, 0)
        };

        // Create the mouse button array
        let buttons = {
            let mut buttons = Vec::with_capacity(5); // There will be 5 buttons, so
                                                     // make a vector with that capacity

            // For each of the five mouse buttons, check whether it's pressed and
            // push the status to the button vector
            for button in &[
                winuser::VK_LBUTTON,
                winuser::VK_RBUTTON,
                winuser::VK_MBUTTON,
                winuser::VK_XBUTTON1,
                winuser::VK_XBUTTON2,
            ] {
                buttons.push(unsafe { winuser::GetAsyncKeyState(*button) as u32 & 0x8000 != 0 });
            }

            // Convert a vector of booleans into an array with a length of 5
            let create_button_arr = |arr: &[bool]| -> [bool; 5] {
                let mut array = [false; 5];
                let buttons = &arr[..array.len()]; // Will panic if the vector is shorter than 5 elements

                array.copy_from_slice(buttons);
                array
            };

            create_button_arr(&buttons)
        };

        MouseState {
            coordinates,
            buttons,
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        let (mut key_codes, mut key_map) = (Vec::with_capacity(256), Vec::with_capacity(256));

        for key in 0..256 {
            key_map.push(unsafe { winuser::GetAsyncKeyState(key) });
        }

        for (ix, byte) in key_map.iter().enumerate() {
            if *byte as u32 & 0x8000 != 0 {
                // If the keycode is matched, then push the resolved
                // key to the key_codes vector
                if let Some(k) = Keycode::keycode_to_key(ix as i32) {
                    key_codes.push(k);
                }
            }
        }

        key_codes
    }
}

impl Default for DeviceState {
    /// Returns a new DeviceState
    fn default() -> Self {
        Self::new()
    }
}
