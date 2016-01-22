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

impl PackFile {
    /// Opens a .pak file and reads the files inside it.
    pub fn open(filename : &str) -> Result<PackFile, &str> {
        let packfile = File::open(filename);
        let mut packfile = PackFile {
            file : match packfile {
                Ok(packfile) => packfile,
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
}