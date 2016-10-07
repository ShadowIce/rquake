#![warn(missing_docs)]

extern crate byteorder;

use std::io;
use self::byteorder::{LittleEndian, ReadBytesExt};
use std::str::from_utf8;

use error;

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
    pub fn read<T:io::Read+io::Seek>(reader : &mut T) -> Result<WadFile, error::ReadError> {
        let start_offset = reader.seek(io::SeekFrom::Current(0)).unwrap();   // will be added to filepos of WAD entries
        let mut headerid = [0u8; 4];
        let headerlen = try!(reader.read(&mut headerid[..]));
        if headerlen < 4 { return Err(error::ReadError::ParseError); }
        if &headerid != &[0x57, 0x41, 0x44, 0x32] { return Err(error::ReadError::ParseError); }

        let mut wadfile = WadFile {
            wadfiles : Vec::new(),
        };

        let numentries = try!(reader.read_i32::<LittleEndian>());
        let diroffset = try!(reader.read_i32::<LittleEndian>()); 

        println!("diroffset = {}", diroffset);
        println!("numentries = {}", numentries);

        try!(reader.seek(io::SeekFrom::Start(start_offset + (diroffset as u64))));

        wadfile.wadfiles.reserve(numentries as usize);
        
        for _ in 0..numentries {
            let offset = try!(reader.read_i32::<LittleEndian>());
            let dsize = try!(reader.read_i32::<LittleEndian>());
            let size = try!(reader.read_i32::<LittleEndian>());
            if size != dsize {
                return Err(error::ReadError::ParseError);
            }
            let filetype = try!(reader.read_u8());
            let compression = try!(reader.read_u8());
            if compression != 0u8 {
                return Err(error::ReadError::ParseError);
            }

            // skip reserved 2 bytes
            try!(reader.seek(io::SeekFrom::Current(2)));

            let mut buf = [0u8; 16];
            try!(reader.read(&mut buf[..]));
            let str_end = buf.iter().position(|c| *c == 0u8).unwrap();
            let filename = match from_utf8(&buf[..str_end]) {
                Err(_) => return Err(error::ReadError::ParseError),
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
    pub fn seek_to_file(&self, wadfile: &mut io::Seek, filename: &str) -> bool {
        if let Some(wadentry) = self.wadfiles.iter().find(|&f| f.name == filename) {
            if let Err(err) = wadfile.seek(io::SeekFrom::Start(wadentry.filepos as u64)) {
                println!("Invalid file position {} for file {}. {}", wadentry.filepos, filename, err);
                return false;
            }
            return true;
        }
        false
    }

}