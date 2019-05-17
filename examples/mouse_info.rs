use device_query::{DeviceQuery, DeviceState, MouseState};

fn main() {
    let device_state = DeviceState::new();
    let mut prev_mouse = MouseState {
        coordinates: (0, 0),
        buttons: [false; 5],
    };

    loop {
        let mouse = device_state.get_mouse();
        if mouse != prev_mouse {
            println!("{:?}", mouse);
        }
        prev_mouse = mouse;
    }
}
