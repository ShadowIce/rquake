#![warn(missing_docs)]

use rquake_common::NativeSoundEngine;

pub struct SoundEngine {
    native_snd : Box<NativeSoundEngine>,
}

impl SoundEngine {
    /// Creates a sound eninge. Only one of those should exist.
    pub fn new(native_snd : Box<NativeSoundEngine>) -> SoundEngine {
        SoundEngine{ native_snd : native_snd, }
    }

    /// Initializes the sound engine.
    pub fn init(&mut self) {
        self.native_snd.init();
    }

    pub fn update(&mut self) {
        
    }

    /// Shuts down any sound processing.
    pub fn shutdown(&mut self) {
        self.native_snd.shutdown();
    }    
}