use std::io::{Read, BufReader, Write, BufWriter};
use std::vec::Vec;

/// This represents the wav format, and when read from will output data that
/// can be interpreted as a wav file.
pub struct WavFormat<'a, R: 'a + Read, W: 'a + Write> {
    l_channel: &'a mut BufReader<R>,
    r_channel: &'a mut BufReader<R>,
    out: &'a mut BufWriter<W>,
    contents: Vec<u8>,
    sample_rate: u32,
}

fn write_little_endian_u16(c: &mut Vec<u8>, val: u16) {
    c.push(val as u8);
    c.push((val >> 8) as u8);
}

fn write_little_endian_u32(c: &mut Vec<u8>, val: u32) {
    c.push(val as u8);
    c.push((val >> 8) as u8);
    c.push((val >> 16) as u8);
    c.push((val >> 24) as u8);
}

impl<'a, R: 'a + Read, W: 'a + Write> WavFormat<'a, R, W> {
    pub fn new(l_channel: &'a mut BufReader<R>,
               r_channel: &'a mut BufReader<R>,
               out: &'a mut BufWriter<W>,
               sample_rate: u32) -> WavFormat<'a, R, W> {
        WavFormat {
            l_channel: l_channel,
            r_channel: r_channel,
            out: out,
            contents: Vec::new(),
            sample_rate: sample_rate,
        }
    }

    pub fn write(&mut self) -> () {
        // make c a reference, since it's shorter to type
        let c = &mut self.contents;
        let channel_count = 2u16;
        let bits_per_sample = 16u16; // 2 bytes per sample
        let byte_rate = (channel_count as u32) 
            * (bits_per_sample as u32 / 8)
            * (self.sample_rate);
        let block_align = channel_count * bits_per_sample / 8;

        // HEADER
        // big endian 4 byte chunk id
        c.push(b'R');
        c.push(b'I');
        c.push(b'F');
        c.push(b'F');

        // little endian 4 byte chunk file size
        // unknown until we unwrap the two channels
        c.push(0);
        c.push(0);
        c.push(0);
        c.push(0);

        // big endian 4 byte format
        c.push(b'W');
        c.push(b'A');
        c.push(b'V');
        c.push(b'E');

        // SUBCHUNK 1
        // big endian 4 byte subchunk1 id
        c.push(b'f');
        c.push(b'm');
        c.push(b't');
        c.push(b' ');

        // little endian 4 byte subchunk 1 size (16)
        c.push(16);
        c.push(0);
        c.push(0);
        c.push(0);

        // little endian 2 byte audio format (1 for PCM)
        c.push(1);
        c.push(0);

        // little endian 2 byte channel count (currently stereo)
        write_little_endian_u16(c, channel_count);

        // little endian 4 byte sample rate
        write_little_endian_u32(c, self.sample_rate);

        // little endian 4 byte byte rate
        write_little_endian_u32(c, byte_rate);
        
        // little endian 4 byte byte rate
        write_little_endian_u32(c, block_align);

        // SUBCHUNK 2
        // big endian 4 byte subchunk2 id
        c.push(b'd');
        c.push(b'a');
        c.push(b't');
        c.push(b'a');
    }
}
