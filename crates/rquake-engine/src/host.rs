#![warn(missing_docs)]

//! Code for the local server setup, teardown and game loop. 
//! 
//! Original source can be found in host.c

use rquake_common::{EventAction,GameResources};
use snd::SoundEngine;

/// Local server instance.
pub struct Host<'a> {
    game_res : &'a mut GameResources,
    snd : &'a mut SoundEngine,
}

impl<'a> Host<'a> {
    /// Creates a new local server instance.
    pub fn new(game_res : &'a mut GameResources, snd : &'a mut SoundEngine) -> Host<'a> {
        Host {
            game_res : game_res,
            snd : snd,
        }
    }
    
    /// Initializes the server. 
    pub fn init(&mut self) {
        self.game_res.add_game_directory("Id1");
        self.snd.init();
    }
    
    /// Runs one frame iteration.
    pub fn frame(&self, timestep : f32, actions : &[EventAction]) {
        
    } 
    
    /// Shuts down the local server.
    pub fn shutdown(&mut self) {
        self.snd.shutdown();
    }
}