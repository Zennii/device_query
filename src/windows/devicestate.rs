use crate::{Keycode, MouseState};
use user32::{GetAsyncKeyState, GetCursorPos};
use winapi::{windef::POINT, winuser};

pub struct DeviceState;

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_pointer(&self) -> MouseState {
        let point = &mut POINT { x: 0, y: 0 }; // Create a new, empty point

        // Create the mouse coordinate tuple
        let coordinates = if unsafe { GetCursorPos(point) != 0 } {
            let (x, y) = *point; // Dereference and disassemble the point into
                                 // the x and y coordinates
            (x, y)
        } else {
            (0, 0)
        };

        // Create the mouse button array
        let buttons = {
            use winuser::{VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, VK_XBUTTON1, VK_XBUTTON2};

            let buttons = Vec::with_capacity(5); // There will be 5 buttons, so
                                                 // make a vector with that capacity

            // For each of the five mouse buttons, check whether it's pressed and
            // push the status to the button vector
            for button in &[VK_LBUTTON, VK_RBUTTON, VK_MBUTTON, VK_XBUTTON1, VK_XBUTTON2] {
                buttons.push(unsafe { GetAsyncKeyState(button) as u32 & 0x8000 != 0 });
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
            key_map.push(unsafe { GetAsyncKeyState(key) });
        }

        for (ix, byte) in key_map.iter().enumerate() {
            if *byte as u32 & 0x8000 != 0 {
                // If the keycode is matched, then push the resolved
                // key to the key_codes vector
                if let Some(k) = self.keycode_to_key(ix as u32) {
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
