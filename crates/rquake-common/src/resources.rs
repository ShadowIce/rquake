#![warn(missing_docs)]

//! Traits for resource and file related stuff.

/// Handles pack files and their content.
pub trait GameResources {
    /// Reads all PAK?.pak files in the given path.
    fn add_game_directory(&mut self, path: &str);
}