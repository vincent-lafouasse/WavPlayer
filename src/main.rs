#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::BufReader;

use rodio::Decoder;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::Source;

const SAMPLE_RATE: u32 = 44_100;

const MP3_PATH: &str = "assets/pra_machucar_meu_coracao.mp3";

fn main() {
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("couldnt open a stream");

    let file: BufReader<File> =
        BufReader::new(File::open(MP3_PATH).expect("couldnt open audio file"));
    let source: Decoder<BufReader<File>> = Decoder::new(file).expect("couldnt decode audio file");

    // feed anything with the Source trait
    // give up ownership of source to background thread
    let _ = stream_handle.play_raw(source.convert_samples());
    // keep main thread alive while background thread runs
    std::thread::sleep(std::time::Duration::from_secs(10));
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
