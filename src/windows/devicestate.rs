use crate::{KeyCode, MouseState};
use winapi::{shared::windef::POINT, um::winuser};

/// The base struct for getting Mouse and Keyboard information,
/// extra methods provided by DeviceQuery
pub struct DeviceState;

impl DeviceState {
    /// Create a new DeviceState
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    /// Query the mouse for it's coordinates and pressed buttons, returned as a MouseState
    pub fn query_mouse(&self) -> MouseState {
        // Create the mouse coordinate tuple
        let point: &mut POINT = &mut POINT { x: 0, y: 0 }; // Create a new, empty point to be filled by GetCursorPos()
        let coordinates = if unsafe { winuser::GetCursorPos(point) != 0 } {
            // Dereference the point and create a tuple from the x and y coordinates
            let point = *point;
            (point.x, point.y)
        } else {
            // Return (0, 0) if GetCursorPos() returns 0
            (0, 0)
        };
        drop(point); // Drop the point

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

        MouseState::from(coordinates, buttons)
    }

    /// Query the keyboard for all pressed keys, returned as a vector of KeyCodes
    pub fn query_keymap(&self) -> Vec<KeyCode> {
        let (mut key_codes, mut key_map) = (Vec::with_capacity(256), Vec::with_capacity(256));

        for key in 0..256 {
            key_map.push(unsafe { winuser::GetAsyncKeyState(key) });
        }

        for (ix, byte) in key_map.iter().enumerate() {
            if *byte as u32 & 0x8000 != 0 {
                // If the keycode is matched, then push the resolved
                // key to the key_codes vector
                if let Some(k) = KeyCode::keycode_to_key(ix as i32) {
                    key_codes.push(k);
                }
            }
        }

        key_codes
    }
}

impl Default for DeviceState {
    /// Create a new DeviceState
    fn default() -> Self {
        Self::new()
    }
}
