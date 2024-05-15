#![allow(unused_variables)]

use cpal::Host;
use cpal::traits::HostTrait;

fn main() {
    let host: Host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");

    println!("Hello, world!");
}
