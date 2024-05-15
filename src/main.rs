#![allow(unused_variables)]

use cpal::traits::HostTrait;
use cpal::Device;
use cpal::Host;
use cpal::StreamConfig;

const SAMPLE_RATE: u32 = 44_100;
//const SAMPLE_FORMAT: cpal::SampleFormat = cpal::SampleFormat::F32;

fn main() {
    let host: Host = cpal::default_host();
    let device: Device = host
        .default_output_device()
        .expect("no output device available");

    let stream_config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE),
        buffer_size: cpal::BufferSize::Default,
    };
}
