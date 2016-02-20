extern crate byteorder; 
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::iter::Iterator;
use std::str::from_utf8;
use byteorder::{LittleEndian, ReadBytesExt};

const MAX_FILES_IN_PACK : i32 = 2048;
const PACKFILE_INFO_LEN : i32 = 64;

struct PackFileInfo {
    name : String,
    filepos : i32,
    filelen : i32,
}

pub struct PackFile {
    file : File,
    packfiles : Vec<PackFileInfo>,
}

pub struct LumpFile {
    pub width : i32,
    pub height : i32,
    pub bitmap : Vec<u32>,
}

fn palette_lookup(index : u8) -> u32 {
    // todo: move to trait and load palette from file.
    let index32 = index as u32;
    index32 + index32 * 256 + index32 * 256 * 256
}

impl PackFile {
    /// Opens a .pak file and reads the file list inside it.
    pub fn open(filename : &str) -> Result<PackFile, &str> {
        let file = File::open(filename);
        let mut packfile = PackFile {
            file : match file {
                Ok(f) => f,
                Err(_) => return Err("Read error"),
            },
            packfiles : Vec::new(),
        };
        
        let mut headerid = [0u8; 4];
        match packfile.file.read(&mut headerid[..]) {
            Ok(res) => if res < 4 { return Err("Read error, could not read header")},
            Err(_) => return Err("Read error"),
        };
        if &headerid != &[0x50, 0x41, 0x43, 0x4B] { return Err("Read error: pack header corrupt"); }
        
        let diroffset = match packfile.file.read_i32::<LittleEndian>() {
            Ok(res) => res,
            Err(_) => return Err("Read error"),
        };
        
        let dirlen = match packfile.file.read_i32::<LittleEndian>() {
            Ok(res) => res,
            Err(_) => return Err("Read error"),
        };

        let numfiles = dirlen / PACKFILE_INFO_LEN;
        
        if numfiles > MAX_FILES_IN_PACK { return Err("Too many files in pack"); }

        println!("dir_offset = {}", diroffset);
        println!("dir_len = {}, numfiles = {}", dirlen, numfiles);
        
        packfile.packfiles.reserve(numfiles as usize);
        
        match packfile.file.seek(SeekFrom::Start(diroffset as u64)) {
            Err(_) => return Err("Read error"),
            _ => {},
        };
        
        for _ in 0..numfiles {
            let mut buf = [0u8; 56];
            match packfile.file.read(&mut buf[..]) {
                Err(_) => return Err("Read error"),
                _ => {},
            }
            let str_end = buf.iter().position(|c| *c == 0u8).unwrap();
            let filename = match from_utf8(&buf[..str_end]) {
                Err(_) => return Err("Read error: filename invalid"),
                Ok(name) => name,
            };
            
            let filepos = match packfile.file.read_i32::<LittleEndian>() {
                Ok(fp) => fp,
                Err(_) => return Err("Read error"),
            };
            let filelen = match packfile.file.read_i32::<LittleEndian>() {
                Ok(fp) => fp,
                Err(_) => return Err("Read error"),
            };
            
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
    pub fn read_lmp(&mut self, name : &str) -> Result<LumpFile, &str> {
        if !name.ends_with(".lmp") {
            println!("File {} has wrong extension. Must be .lmp.", name);
            return Err("Wrong file extension. Must be .lmp");
        }
        
        if !self.seek_to_file(name) {
            println!("File {} not found", name);
            return Err("File not found.");
        }
        
        let width = match self.file.read_i32::<LittleEndian>() {
            Ok(w) => w,
            Err(_) => return Err("Read error"),
        };

        let height = match self.file.read_i32::<LittleEndian>() {
            Ok(h) => h,
            Err(_) => return Err("Read error"),
        };
 
        let mut buffer = vec![0; (width * height) as usize];
        if let Err(err) = self.file.read(&mut buffer) {
            println!("Read error on file {}, {}", name, err);
            return Err("Read error");
        }
        
        // todo: add proper palette lookup
        let bitmap = buffer.iter().map(|&x| palette_lookup(x)).collect();

        println!("Width: {}, Height: {}", width, height);
        
        Ok(LumpFile {
            width : width,
            height : height,
            bitmap : bitmap,
        })
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
        let mut packfile = PackFile::open("Id1/PAK0.PAK");
    }
    
    #[test]
    fn read_lmp_file() {
        let mut packfile = PackFile::open("Id1/PAK0.PAK").unwrap();
        let pause_bitmap = packfile.read_lmp("gfx/pause.lmp").unwrap();
        assert_eq!(pause_bitmap.width, 128);
        assert_eq!(pause_bitmap.height, 24);
    }
}