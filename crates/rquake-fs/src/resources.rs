#![warn(missing_docs)]

extern crate rquake_common;

use packfile::PackFile;
use self::rquake_common::GameResources;

/// Handles game resources.
pub struct GameResourcesImpl {
    packfiles : Vec<PackFile>,
}

impl GameResourcesImpl {
    /// Constructor
    pub fn new() -> GameResourcesImpl {
        GameResourcesImpl {
            packfiles : Vec::new(),
        }
    }
}

impl GameResources for GameResourcesImpl {
    /// Reads all PAK?.pak files in the given path.
    fn add_game_directory(&mut self, path: &str) {
        let mut i = 0;
        loop {
            let filepath = format!("{}/pak{}.pak", path, i);
            println!("Trying to read {}", &filepath);
            let new_packfile = PackFile::open(&filepath);
            match new_packfile {
                Ok(new_packfile) => self.packfiles.push(new_packfile),
                Err(_) => break,
            }
            i += 1;
        }
        println!("Read {} pak files", self.packfiles.len());
    }
}