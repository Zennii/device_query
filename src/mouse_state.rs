/// A simple structure containing the current mouse coordinates and the
/// state of each mouse button that we can query. Currently, Windows and
/// Linux provide nice ways to query five mouse buttons.
///
/// `buttons` will contain an array of the five mouse buttons, with `true` meaning that the button
/// is pressed and `false` meaning that the button is not pressed. Index 0 is mouse button 1, Index 1 is button 2, etc.  
/// ```rust
/// # fn example_buttons() -> Result<(), ()> {
/// # use device_query::{DeviceQuery, DeviceState, MouseState};
/// # let device_state = DeviceState::new();
/// let mouse: MouseState = device_state.get_mouse(); // get_mouse() from the `DeviceQuery` trait
/// println!("{:?}", mouse.buttons()); // Prints something along the lines of
///                                    // `[false, true, false, false, false]`, depending on what buttons are
///                                    // pressed. Index 0 is mouse button 1, Index 1 is button 2, etc.
/// # Ok(())
/// # }
/// ```
///
/// `coordinates` will contain a tuple of the x and y coordinates of the cursor  
/// ```rust
/// # fn example_coordinates() -> Result<(), ()> {
/// # use device_query::{DeviceQuery, DeviceState, MouseState};
/// # let device_state = DeviceState::new();
/// let mouse: MouseState = device_state.get_mouse(); // get_mouse() from the `DeviceQuery` trait
/// println!("{:?}", mouse.coordinates()); // Prints something along the lines of `(100, 100)`, depending on
///                                        // where your mouse is
/// # Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct MouseState {
    coordinates: (i32, i32),
    buttons: [bool; 5],
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

    /// Get a mouse button
    pub fn get_button(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Right => self.buttons[0],
            MouseButton::Left => self.buttons[1],
            MouseButton::Middle => self.buttons[2],
            MouseButton::Four => self.buttons[3],
            MouseButton::Five => self.buttons[4],
        }
    }

    /// Get a vector of the currently activated MouseButtons
    pub fn get_buttons(&self) -> Vec<MouseButton> {
        let mut buttons = Vec::with_capacity(5);
        let codes = [
            MouseButton::Right,
            MouseButton::Left,
            MouseButton::Middle,
            MouseButton::Four,
            MouseButton::Five,
        ];

        for (i, button) in (&self.buttons).iter().enumerate() {
            if *button {
                buttons.push(codes[i].clone());
            }
        }

        buttons
    }

    /// Create a MouseState from a coordinate tuple and a button array
    pub fn from(coordinates: (i32, i32), buttons: [bool; 5]) -> Self {
        Self {
            coordinates,
            buttons,
        }
    }
}

/// Allows access to mouse buttons in a named way
/// via MouseState::get_button()
#[derive(PartialEq, Debug, Clone)]
pub enum MouseButton {
    Right,
    Left,
    Middle,
    /// Also known as XButton1
    Four,
    /// Also known as XButton2
    Five,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_test() {
        let test_mouse = MouseState::from((100, 100), [false; 5]);

        assert!(
            test_mouse.coordinates == test_mouse.coordinates()
                && test_mouse.coordinates == (100, 100)
                && test_mouse.coordinates() == (100, 100)
        );
    }

    #[test]
    fn button_test() {
        let test_mouse = MouseState::from((100, 100), [false; 5]);

        assert!(
            test_mouse.buttons == test_mouse.buttons()
                && test_mouse.buttons == [false; 5]
                && test_mouse.buttons() == [false; 5]
        );
    }

    #[test]
    fn named_button_test() {
        let test_mouse = MouseState::from((100, 100), [true, false, true, false, true]);

        assert_eq!(test_mouse.get_button(MouseButton::Right), true);
        assert_eq!(test_mouse.get_button(MouseButton::Left), false);
        assert_eq!(test_mouse.get_button(MouseButton::Middle), true);
        assert_eq!(test_mouse.get_button(MouseButton::Four), false);
        assert_eq!(test_mouse.get_button(MouseButton::Five), true);
    }

    #[test]
    fn get_buttons_test() {
        let test_mouse = MouseState::from((100, 100), [false; 5]);

        assert_eq!(test_mouse.get_buttons(), Vec::default());

        let test_mouse = MouseState::from((100, 100), [true, false, true, false, true]);

        assert_eq!(
            test_mouse.get_buttons(),
            vec![MouseButton::Right, MouseButton::Middle, MouseButton::Five]
        );
    }
}
