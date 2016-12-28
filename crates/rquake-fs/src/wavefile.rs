#![warn(missing_docs)]

//extern crate riff_wave;

use std::io::{Read,Seek};
use riff_wave::WaveReader;

use error;

/// RIFF-WAVE file data
pub struct Sound {
    /// 16 bit samples of a wave file.
    pub samples : Vec<i16>,
}

impl Sound {
    /// Reads the samples of a 16 bit riff-wave file.
    pub fn read<T : Read + Seek>(reader : &mut T) -> Result<Sound, error::ReadError> {
        let mut samples : Vec<i16> = Vec::new();
        let mut wav_reader = WaveReader::new(reader)?;
        while let Ok(sample) = wav_reader.read_sample_i16() {
            samples.push(sample);
        }
        Ok(Sound { samples : samples })
    }
}