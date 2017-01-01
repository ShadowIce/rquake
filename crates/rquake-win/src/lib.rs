#![crate_type= "lib"]

extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate rquake_common;
extern crate gdi32;

pub use window::WinWindow;
pub use sound::DirectSoundEngine;

mod window;
mod sound;


