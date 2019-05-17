/// A simple structure containing the current mouse coordinates and the
/// state of each mouse button that we can query. Currently, Windows and
/// Linux provide nice ways to query five mouse buttons.
#[derive(Debug, PartialEq, Clone)]
pub struct MouseState {
    /// `coords` will contain a tuple of the x and y coordinates of the cursor  
    /// ```rust
    /// # use device_query::{DeviceQuery, DeviceState, MouseState};
    /// # let device_state = DeviceState::new();
    /// let mouse: MouseState = device_state.get_mouse(); // get_mouse() from the `DeviceQuery` trait
    /// println!("{:?}", mouse.coordinates); // Prints something along the lines of `(100, 100)`, depending on where
    ///                                 // Your mouse is
    /// ```
    ///
    pub coordinates: (i32, i32),

    /// `buttons` will contain an array of the five mouse buttons, with `true` meaning that the button
    /// is pressed and `false` meaning that the button is not pressed. Index 0 is mouse button 1, Index 1 is button 2, etc.  
    /// ```rust
    /// # use device_query::{DeviceQuery, DeviceState, MouseState};
    /// # let device_state = DeviceState::new();
    /// let mouse: MouseState = device_state.get_mouse(); // get_mouse() from the `DeviceQuery` trait
    /// println!("{:?}", mouse.buttons); // Prints something along the lines of
    ///                                  // `[false, true, false, false, false]`, depending on what buttons are
    ///                                  // pressed. Index 0 is mouse button 1, Index 1 is button 2, etc.
    /// ```
    pub buttons: [bool; 5],
}

impl MouseState {
    /// Get the coordinates of the mouse cursor
    pub fn coordinates(&self) -> (i32, i32) {
        self.coordinates
    }

    /// Get an array containing the state of the mouse buttons
    pub fn buttons(&self) -> [bool; 5] {
        self.buttons
    }
}
