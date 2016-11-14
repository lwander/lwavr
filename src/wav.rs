use std::io::{Read, BufReader};

/// This represents the wav format, and when read from will output data that 
/// can be interpreted as a wav file.
pub struct WavFormat<'a, R: 'a + Read> {
    l_channel: &'a mut BufReader<R>,
    r_channel: &'a mut BufReader<R>,
}

impl<'a, R: 'a + Read> WavFormat<'a, R> {
    pub fn new(l_channel: &'a mut BufReader<R>, 
               r_channel: &'a mut BufReader<R>) -> WavFormat<'a, R> {
        WavFormat {
            l_channel: l_channel,
            r_channel: r_channel,
        }
    }
}
