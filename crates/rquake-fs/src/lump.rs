#![warn(missing_docs)]

extern crate byteorder;

use std::io::Read;
use self::byteorder::{LittleEndian, ReadBytesExt};

use error;

/// Palette for color lookup from 8bit color index to 32bit RGBA.
pub struct Palette {
    palette : [u32;256],
}

impl Palette {
    /// Reads a palette of 256 RGB colors.
    pub fn read(reader : &mut Read) -> Result<Palette, error::ReadError> {
        let mut pal = [0u8;256 * 3];
        try!(reader.read(&mut pal));
        
        let mut pal32 = [0u32; 256];
        let mut pal_iter = pal.iter();
        for value in pal32.iter_mut() {
            *value = *pal_iter.next().unwrap() as u32;
            *value = *value * 256 + *pal_iter.next().unwrap() as u32;
            *value = *value * 256 + *pal_iter.next().unwrap() as u32;
        }
        
        Ok(Palette { palette : pal32 })
    }

    fn palette_lookup(&self, index : u8) -> u32 {
        self.palette[index as usize]
    }
}

/// Lump picture data.
pub struct Picture {
    /// Width of the bitmap.
    pub width : i32,
    
    /// Height of the bitmap.
    pub height : i32,
    
    /// Bitmap content.
    pub bitmap : Vec<u32>,
}

impl Picture {

    /// Reads a picture lmp (lump) file and converts it to RGBA using a palette.
    pub fn read(reader : &mut Read, pal : &Palette) -> Result<Picture, error::ReadError> {
        let width = try!(reader.read_i32::<LittleEndian>());
        let height = try!(reader.read_i32::<LittleEndian>());
        let mut buffer = vec![0; (width * height) as usize];
        try!(reader.read(&mut buffer));
        
        let bitmap = buffer.iter().map(|&x| pal.palette_lookup(x)).collect();

        println!("Width: {}, Height: {}", width, height);
        
        Ok(Picture {
            width : width,
            height : height,
            bitmap : bitmap,
        })
    }
}