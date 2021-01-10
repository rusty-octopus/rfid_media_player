extern crate rodio;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

#[cfg(not(tarpaulin_include))]
fn play(path: &str) -> (Sink, OutputStream, OutputStreamHandle) {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.1);
    sink.append(source);
    sink.play();
    (sink, stream, stream_handle)
}

#[cfg(not(tarpaulin_include))]
fn do_play2() {
    let (mut sink, mut _stream, mut _stream_handle) = play("sound.flac");

    let mut counter = 1;

    loop {
        let is_playing = !sink.empty();
        println!("Still playing? {}", is_playing);
        if counter == 5 {
            sink.stop();
            println!("Stopped");
        }
        if counter == 7 {
            let (sink_new, stream_new, stream_handle_new) = play("sound2.wav");
            sink = sink_new;
            _stream = stream_new;
            _stream_handle = stream_handle_new;
            println!("Started new sound");
        }
        if counter == 25 {
            println!("Should have stopped playing");
            println!("Still playing? {}", is_playing);
            break;
        }
        sleep(Duration::from_secs(2));
        counter += 1;
    }

    println!("sound over");
}

#[cfg(not(tarpaulin_include))]
fn main() {
    do_play2();
}
