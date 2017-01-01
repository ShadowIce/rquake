#![crate_type= "lib"]

pub use system::Window;
pub use system::BackBuffer;
pub use system::ToggleFullscreen;
pub use system::NativeSoundEngine;
pub use utils::Timer;
pub use types::EventAction;
pub use resources::GameResources;

mod system;
mod utils;
mod types;
mod resources;