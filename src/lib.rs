//! A simple library for querying mouse and keyboard state without requiring
//! an active window. Currently works in Windows and Linux.
//!
//! ```rust
//! # fn example_usage() -> Result<(), ()> {
//! // A glob import will import the same things, this is just an explicit import
//! use device_query::{DeviceQuery, DeviceState, MouseState, KeyCode};
//!
//! let device_state = DeviceState::new(); // DeviceState::default() is also valid  
//!
//! let mouse: MouseState = device_state.get_mouse();
//! println!("Current Mouse Coordinates: {:?}", mouse.coordinates());  
//!
//! let keys: Vec<KeyCode> = device_state.get_keys();
//! println!("Is A pressed? {}", keys.contains(&KeyCode::A));
//! # Ok(())
//! # }
//! ```

mod mouse_state;
pub use mouse_state::{MouseButton, MouseState};

mod device_query;
pub use crate::device_query::DeviceQuery;

// Exposes the linux version for linux builds
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{DeviceState, KeyCode};

// Exposes the windows version for windows builds
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{DeviceState, KeyCode};
