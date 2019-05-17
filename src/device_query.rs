use crate::{DeviceState, KeyCode, MouseState};

/// A trait to allow the fetching of mouse and key data
pub trait DeviceQuery {
    fn get_mouse(&self) -> MouseState;
    fn get_keys(&self) -> Vec<KeyCode>;
}

impl DeviceQuery for DeviceState {
    /// Query for the current mouse position and mouse button state.
    fn get_mouse(&self) -> MouseState {
        self.query_pointer()
    }

    /// Query for all keys that are currently pressed down.
    fn get_keys(&self) -> Vec<KeyCode> {
        self.query_keymap()
    }
}
