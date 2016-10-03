#![warn(missing_docs)]

extern crate byteorder;

use std::io::{Read,Seek,SeekFrom};
use self::byteorder::{LittleEndian, ReadBytesExt};
use std::str::from_utf8;

enum WadFileType {
    Unknown,
    ColorPalette,   // 0x40=	'@'=	Color Palette
    Picture,        // 0x42=	'B'=	Pictures for status bar
    MipTexture,     // 0x44=	'D'=	Used to be Mip Texture
    ConsolePicture, // 0x45=	'E'=	Console picture (flat)
}

/// Contains information about each file inside a wad file.
struct WadFileInfo {
    name        : String,
    filepos     : i32,
    filelen     : i32,
    filetype    : WadFileType,
}

/// Contains the list of files inside a wad file.
/// WadFile doesn't own a file handle, that is owned by PackFile.
pub struct WadFile {
    wadfiles : Vec<WadFileInfo>
}

impl WadFile {

    /// Reads the file list in a wad file. The reader must point to the beginning of the wad file.
    pub fn read<T:Read+Seek>(reader : &mut T) -> Result<WadFile, &'static str> {
        let start_offset = reader.seek(SeekFrom::Current(0)).unwrap();   // will be added to filepos of WAD entries
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

        if let Err(_) = reader.seek(SeekFrom::Start(start_offset + (diroffset as u64))) {
            return Err("Read error");
        }

        wadfile.wadfiles.reserve(numentries as usize);
        
        for _ in 0..numentries {
            let offset = match reader.read_i32::<LittleEndian>() {
                Ok(res) => res,
                Err(_) => return Err("Read error"),
            };
            let dsize = match reader.read_i32::<LittleEndian>() {
                Ok(res) => res,
                Err(_) => return Err("Read error"),
            };
            let size = match reader.read_i32::<LittleEndian>() {
                Ok(res) => res,
                Err(_) => return Err("Read error"),
            };
            if size != dsize {
                return Err("Unexpected mismatch between size and dsize in WAD file");
            }
            let filetype = match reader.read_u8() {
                Ok(res) => res,
                Err(_) => return Err("Read error"),
            };
            let compression = match reader.read_u8() {
                Ok(res) => res,
                Err(_) => return Err("Read error"),
            };
            if compression != 0u8 {
                return Err("Compressed files in WAD not supported");
            }
            // skip reserved 2 bytes
            if let Err(_) = reader.seek(SeekFrom::Current(2)) {
                return Err("Read error");
            }
            let mut buf = [0u8; 16];
            match reader.read(&mut buf[..]) {
                Err(_) => return Err("Read error"),
                _ => {},
            }
            let str_end = buf.iter().position(|c| *c == 0u8).unwrap();
            let filename = match from_utf8(&buf[..str_end]) {
                Err(_) => return Err("Read error: filename invalid"),
                Ok(name) => name,
            };
            wadfile.wadfiles.push(WadFileInfo {
                name : filename.to_string(),
                filepos : offset + (start_offset as i32),
                filelen : size,
                filetype : match filetype {
                    0x40 => WadFileType::ColorPalette,
                    0x42 => WadFileType::Picture,
                    0x44 => WadFileType::MipTexture,
                    0x45 => WadFileType::ConsolePicture,
                    _ => WadFileType::Unknown,
                }
            });
        }

        Ok(wadfile)
    }

    /// Seeks to the position of a file inside a WAD file.
    pub fn seek_to_file(&self, wadfile: &mut Seek, filename: &str) -> bool {
        if let Some(wadentry) = self.wadfiles.iter().find(|&f| f.name == filename) {
            if let Err(err) = wadfile.seek(SeekFrom::Start(wadentry.filepos as u64)) {
                println!("Invalid file position {} for file {}. {}", wadentry.filepos, filename, err);
                return false;
            }
            return true;
        }
        false
    }

}