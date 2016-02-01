#![warn(missing_docs)]

//! Code for the local server setup, teardown and game loop. 
//! 
//! Original source can be found in host.c

/// Local server instance.
pub struct Host;

impl Host {
    /// Creates a new local server instance.
    pub fn new() -> Host {
        Host
    }
    
    /// Initializes the server. 
    pub fn init(&self) {
        
    }
    
    /// Runs one frame iteration.
    pub fn frame(&self, timestep : f32) {
        
    } 
    
    /// Shuts down the local server.
    pub fn shutdown(&self) {
        
    }
}