#![warn(missing_docs)]

//! Traits for OS related things.

use types::EventAction;

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
    fn handle_message(&mut self) -> Vec<EventAction>;
    
    /// Returns the back buffer of the Window. This is
    /// used to draw stuff into.
    fn get_backbuffer(&mut self) -> &mut BackBuffer;
    
    /// Renders the back buffer to the window.
    fn render(&mut self);
}

/// Trait representing a bitmap buffer that can be drawn into. 
pub trait BackBuffer {
    /// Returns the buffer as slice.
    fn get_buffer(&mut self) -> &mut [u32];
    
    /// Returns the width of the back buffer.
    fn get_width(&self) -> u32;
    
    /// Returns the height of the buffer.
    fn get_height(&self) -> u32;
}

/// Trait required to switch between fullscreen and windowed mode.
pub trait ToggleFullscreen {
    /// Switches between fullscreen and windowed mode.
    fn toggle_fullscreen(&mut self);
}