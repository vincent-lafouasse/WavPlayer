#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;
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

    let stream = device.build_output_stream(
        &stream_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // react to stream events and read or write stream data here.
        },
        move |err| {
            // react to errors here.
        },
        None, // None=blocking, Some(Duration)=timeout
    );
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
