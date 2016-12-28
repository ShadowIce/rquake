#![warn(missing_docs)]

//! Handling of PAK?.pak files.

extern crate byteorder;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::iter::Iterator;
use std::str::from_utf8;

use self::byteorder::{LittleEndian, ReadBytesExt};

use lump::{Picture, Palette};
use wadfile::WadFile;
use wavefile::Sound;
use error;

const MAX_FILES_IN_PACK : i32 = 2048;
const PACKFILE_INFO_LEN : i32 = 64;

struct PackFileInfo {
    name : String,
    filepos : i32,
    filelen : i32,
}

/// TODO: make non-public
pub struct PackFile {
    file : File,
    packfiles : Vec<PackFileInfo>,
}

impl PackFile {
    /// Opens a .pak file and reads the file list inside it.
    pub fn open(filename : &str) -> Result<PackFile, error::ReadError> {
        let file = File::open(filename);
        let mut packfile = PackFile {
            file : match file {
                Ok(f) => f,
                Err(_) => return Err(error::ReadError::FileNotFound),
            },
            packfiles : Vec::new(),
        };
        
        let mut headerid = [0u8; 4];
        let headerlen = packfile.file.read(&mut headerid[..])?;
        if headerlen < 4 { return Err(error::ReadError::ParseError); }
        if &headerid != &[0x50, 0x41, 0x43, 0x4B] { return Err(error::ReadError::ParseError); }
        
        let diroffset = packfile.file.read_i32::<LittleEndian>()?;
        let dirlen = packfile.file.read_i32::<LittleEndian>()?;

        let numfiles = dirlen / PACKFILE_INFO_LEN;
        if numfiles > MAX_FILES_IN_PACK { return Err(error::ReadError::ParseError); }

        println!("dir_offset = {}", diroffset);
        println!("dir_len = {}, numfiles = {}", dirlen, numfiles);
        
        packfile.packfiles.reserve(numfiles as usize);
        
        packfile.file.seek(SeekFrom::Start(diroffset as u64))?;
        
        for _ in 0..numfiles {
            let mut buf = [0u8; 56];
            packfile.file.read(&mut buf[..])?;
            let str_end = buf.iter().position(|c| *c == 0u8).unwrap();
            let filename = match from_utf8(&buf[..str_end]) {
                Err(_) => return Err(error::ReadError::ParseError),
                Ok(name) => name,
            };
            
            let filepos = packfile.file.read_i32::<LittleEndian>()?;
            let filelen = packfile.file.read_i32::<LittleEndian>()?;
            
            println!("filename = {}, pos = {}, len = {}, base = {}", filename, filepos, filelen, filebase(filename));
            packfile.packfiles.push(PackFileInfo { 
                name : filename.to_string(),
                filepos : filepos,
                filelen : filelen,
            });
        }
        
        Ok(packfile)
    }
    
    /// Reads a lmp (lump) file and converts it to RGBA.
    pub fn read_lmp(&mut self, name : &str, pal : &Palette) -> Result<Picture, error::ReadError> {
        if !name.ends_with(".lmp") {
            println!("File {} has wrong extension. Must be .lmp.", name);
            return Err(error::ReadError::ParseError);
        }
        
        if !self.seek_to_file(name) {
            println!("File {} not found", name);
            return Err(error::ReadError::FileNotFound);
        }

        Picture::read(&mut self.file, &pal)
    }
    
    /// TODO: make non-public
    pub fn read_palette(&mut self) -> Result<Palette, error::ReadError> {
        if !self.seek_to_file("gfx/palette.lmp") {
            println!("Palette file not found.");
            return Err(error::ReadError::FileNotFound);
        }

        Palette::read(&mut self.file)        
    }
    
    /// reads the directory structure of a wad file
    pub fn read_wad(&mut self, name : &str) -> Result<WadFile, error::ReadError> {
        if !self.seek_to_file(name) {
            println!("File {} not found", name);
            return Err(error::ReadError::FileNotFound);
        }

        WadFile::read(&mut self.file)
    }

    /// Reads a wave file.
    pub fn read_wave(&mut self, name : &str) -> Result<Sound, error::ReadError> {
        if !self.seek_to_file(name) {
            println!("File {} not found", name);
            return Err(error::ReadError::FileNotFound);
        }
        Sound::read(&mut self.file)
    }

    fn seek_to_file(&mut self, name : &str) -> bool {
        if let Some(pf) = self.packfiles.iter().find(|&f| f.name == name) {
            if let Err(err) = self.file.seek(SeekFrom::Start(pf.filepos as u64)) {
                println!("Invalid file position {} for file {}. {}", pf.filepos, name, err);
                return false;
            }
            return true;
        }
        false
    }
}

/// Returns the file name without path and extension.
fn filebase(name : &str) -> &str {
    let mut start = name.rfind('/').unwrap_or(0);
    if start > 0 || (!name.is_empty() && name.starts_with("/")) { 
        start += 1;
    }
    let end = name.rfind('.').unwrap_or(name.len());
    if end - start >= 2 {
        return &name[start..end];
    }
    "?model?"
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn get_base_name() {
        assert_eq!(super::filebase(""), "?model?");
        assert_eq!(super::filebase("sound/items/r_item1.wav"), "r_item1");
        assert_eq!(super::filebase("gfx.wad"), "gfx");
    }

    #[test]
    fn open_pak_file() {
        let mut packfile = PackFile::open("../../Id1/PAK0.PAK").unwrap();
    }
    
    #[test]
    fn read_lmp_file() {
        let mut packfile = PackFile::open("../../Id1/PAK0.PAK").unwrap();
        let pal = packfile.read_palette().unwrap();
        let pause_bitmap = packfile.read_lmp("gfx/pause.lmp", &pal).unwrap();
        assert_eq!(pause_bitmap.width, 128);
        assert_eq!(pause_bitmap.height, 24);
    }
}