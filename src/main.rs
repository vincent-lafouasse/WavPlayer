#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rodio::OutputStream;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
}
