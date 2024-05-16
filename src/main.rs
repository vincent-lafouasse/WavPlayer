#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;
use cpal::Device;
use cpal::Host;
use cpal::SupportedStreamConfig;

fn main() {
    let (_host, device, config) = setup_device().unwrap();
    let stream = device.build_output_stream(
        &config.config(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // react to stream events and read or write stream data here.
        },
        move |err| {
            // react to errors here.
        },
        None, // None=blocking, Some(Duration)=timeout
    );
}

fn setup_device() -> Result<(Host, Device, SupportedStreamConfig), &'static str> {
    let host: Host = cpal::default_host();
    let device: Device = match host.default_output_device() {
        Some(device) => device,
        None => return Err("no output device available"),
    };

    let stream_config: SupportedStreamConfig = match device.default_output_config() {
        Ok(config) => config,
        Err(_) => return Err("couldnt find default stream config"),
    };

    Ok((host, device, stream_config))
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
