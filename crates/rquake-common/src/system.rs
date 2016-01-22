#![warn(missing_docs)]

//! Contains traits for OS related things.

/// Trait representing the main window.
pub trait Window {
    /// Shows the main window.
    fn show_window(&self);
    
    /// Returns true until the program should terminate.
    /// The flag will be set to false if the window is destroyed
    /// and the message loop retrieves the final message. 
    fn is_running(&self) -> bool;
    
    /// Should be called for each iteration of the game loop
    /// to handle window/system messages.
    fn handle_message(&mut self);
}