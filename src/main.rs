#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::BufReader;

use rodio::OutputStream;
use rodio::OutputStreamHandle;

const SAMPLE_RATE: u32 = 44_100;

const MP3_PATH: &str = "assets/pra_machucar_meu_coracao.mp3";

fn main() {
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("couldnt open a stream");

    let file: BufReader<File> =
        BufReader::new(File::open(MP3_PATH).expect("couldnt open audio file"));
    print_type_of(&file);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
