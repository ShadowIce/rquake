#![warn(missing_docs)]

use std::io::{Read,Seek};
use riff_wave::WaveReader;

use error;

/// RIFF-WAVE file data
pub struct Sound {
    /// unsigned 8 bit samples of a wave file.
    pub samples : Vec<u8>,
}

impl Sound {
    /// Reads the samples of a 8 bit riff-wave file.
    pub fn read<T : Read + Seek>(reader : &mut T) -> Result<Sound, error::ReadError> {
        let mut samples : Vec<u8> = Vec::new();
        let mut wav_reader = WaveReader::new(reader)?;
        while let Ok(sample) = wav_reader.read_sample_u8() {
            samples.push(sample);
        }
        Ok(Sound { samples : samples })
    }
}