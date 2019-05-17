use device_query::{DeviceQuery, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    let mut prev_keys = Vec::default();

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            println!("{:?}", keys);
        }
        prev_keys = keys;
    }
}
