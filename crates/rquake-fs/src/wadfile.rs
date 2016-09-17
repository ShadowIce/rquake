#![warn(missing_docs)]

extern crate byteorder;

use std::io::Read;
use self::byteorder::{LittleEndian, ReadBytesExt};

/// Contains information about each file inside a wad file.
pub struct WadFileInfo {

}

/// Contains the list of files inside a wad file.
pub struct WadFile {
    //file : File,
    wadfiles : Vec<WadFileInfo>
}

impl WadFile {

    /// Reads the file list in a wad file. The reader must point to the beginning of the wad file.
    pub fn read(reader : &mut Read) -> Result<WadFile, &'static str> {
        let mut headerid = [0u8; 4];
        match reader.read(&mut headerid[..]) {
            Ok(res) => if res < 4 { return Err("Read error, could not read WAD header")},
            Err(_) => return Err("Read error"),
        };
        if &headerid != &[0x57, 0x41, 0x44, 0x32] { return Err("Read error: WAD header corrupt/unknown"); }

        let mut wadfile = WadFile {
            wadfiles : Vec::new(),
        };

        let numentries = match reader.read_i32::<LittleEndian>() {
            Ok(res) => res,
            Err(_) => return Err("Read error"),
        };

        let diroffset = match reader.read_i32::<LittleEndian>() {
            Ok(res) => res,
            Err(_) => return Err("Read error"),
        };

        println!("diroffset = {}", diroffset);
        println!("numentries = {}", numentries);

        wadfile.wadfiles.reserve(numentries as usize);

        Ok(wadfile)
    }

}