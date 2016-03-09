#![crate_type= "lib"]

pub use system::Window;
pub use system::BackBuffer;
pub use system::ToggleFullscreen;
pub use utils::Timer;
pub use types::EventAction;

mod system;
mod utils;
mod types;
