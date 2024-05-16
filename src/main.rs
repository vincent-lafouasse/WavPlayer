#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;
use cpal::Device;
use cpal::Host;
use cpal::SampleFormat;
use cpal::SupportedStreamConfig;

fn main() {
    let (_host, device, config) = setup_device().unwrap();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let stream = match config.sample_format() {
        SampleFormat::F32 => {
            device.build_output_stream(&config.config(), audio_callback::<f32>, err_fn, None)
        }
        SampleFormat::U32 => {
            device.build_output_stream(&config.config(), audio_callback::<u32>, err_fn, None)
        }
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
    .expect("Failed to create output stream");

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    // keep main thread alive while audio thread is running
}

fn audio_callback<T: cpal::Sample>(data: &mut [T], callback_info: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = cpal::Sample::EQUILIBRIUM;
    }
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
