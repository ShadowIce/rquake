#![warn(missing_docs)]

//! Code for the local server setup, teardown and game loop. 
//! 
//! Original source can be found in host.c

extern crate rquake_common;

use self::rquake_common::{EventAction,GameResources};

/// Local server instance.
pub struct Host<'a> {
    game_res : &'a mut GameResources,
}

impl<'a> Host<'a> {
    /// Creates a new local server instance.
    pub fn new(game_res : &mut GameResources) -> Host {
        Host {
            game_res : game_res,
        }
    }
    
    /// Initializes the server. 
    pub fn init(&mut self) {
        self.game_res.add_game_directory("Id1");
    }
    
    /// Runs one frame iteration.
    pub fn frame(&self, timestep : f32, actions : &[EventAction]) {
        
    } 
    
    /// Shuts down the local server.
    pub fn shutdown(&self) {
        
    }
}