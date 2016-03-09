#![warn(missing_docs)]

//! Types used by all crates.

/// Enum for all actions triggered by input/system events.
pub enum EventAction {
    
    /// Switch between fullscreen and windowed mode.
    ToggleFullscreen,
}