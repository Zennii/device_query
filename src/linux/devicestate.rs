use crate::{Keycode, MouseState};
use std::{ptr, slice};
use x11::xlib;

pub struct DeviceState {
    display: *mut xlib::Display,
}

impl DeviceState {
    /// Create a new DeviceState
    pub fn new() -> DeviceState {
        unsafe {
            xlib::XInitThreads();

            DeviceState {
                display: xlib::XOpenDisplay(ptr::null()),
            }
        }
    }

    pub fn query_pointer(&self) -> MouseState {
        let (mut win_x, mut win_y, mut mask_return) = (0, 0, 0);

        unsafe {
            let root = xlib::XDefaultRootWindow(self.display);

            xlib::XQueryPointer(
                self.display,
                root,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );
        }

        // Create the mouse button array
        let buttons = {
            use xlib::{Button1Mask, Button2Mask, Button3Mask, Button4Mask, Button5Mask};

            let mut buttons = Vec::with_capacity(5); // There will be 5 buttons, so
                                                     // make a vector with that capacity

            // For each of the five mouse buttons, check whether it's pressed and
            // push the status to the button vector
            for button in &[
                Button1Mask,
                Button2Mask,
                Button3Mask,
                Button4Mask,
                Button5Mask,
            ] {
                buttons.push(mask_return & button > 0);
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

        MouseState::from((win_x, win_y), buttons);
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        let mut key_codes = Vec::new(); // Create vector to hold all key codes
        let key_map: *mut i8 = [0; 32].as_mut_ptr();

        unsafe {
            xlib::XQueryKeymap(self.display, key_map);
        }

        for (ix, byte) in unsafe { slice::from_raw_parts(key_map, 32).iter().enumerate() } {
            for bit in 0_u8..8_u8 {
                let bitmask = 1 << bit;

                if byte & bitmask != 0 {
                    let keycode = ix as u8 * 8 + bit;
                    let mut key_syms: i32 = 0;

                    unsafe {
                        let key_sym =
                            xlib::XGetKeyboardMapping(self.display, keycode, 1, &mut key_syms);

                        for ks in slice::from_raw_parts(key_sym, key_syms as usize).iter() {
                            // Attempt to match keycode against keys and if
                            // the key is matched push to the key_codes vector
                            if let Some(k) = Keycode::keysym_to_key(*ks as u32) {
                                key_codes.push(k)
                            };
                        }

                        // Free the memory allocated by XGetKeyboardMapping.
                        xlib::XFree(key_sym as *mut std::ffi::c_void);
                    }
                }
            }
        }

        key_codes.dedup();

        key_codes
    }
}

impl Default for DeviceState {
    /// Create a new DeviceState
    fn default() -> Self {
        Self::new()
    }
}
