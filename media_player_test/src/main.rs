extern crate rodio;
use rodio::Decoder;
use rodio::Sink;
use rodio::Source;
use rodio::OutputStreamHandle;
use rodio::OutputStream;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

fn do_play() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    //let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open("sound.flac").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    //let (sink, output_queue) = Sink::new_idle();
    sink.set_volume(0.1);
    //let sink = stream_handle.play_raw(source.convert_samples()).unwrap();
    //let source = rodio::source::SineWave::new(440);
    //sink.append(source);
    sink.append(source);
    let is_playing = !sink.empty();
    println!("Still playing? {}", is_playing);
    println!("Is paused? {}", sink.is_paused());
    sink.play();
    let mut counter = 0;
    loop {
        sleep(Duration::from_secs(2));
        let is_playing = !sink.empty();
        println!("Still playing? {}", is_playing);
        println!("Is paused? {}", sink.is_paused());
        if counter == 5 {
            sink.stop();
            let is_playing = !sink.empty();
            println!("Still playing? {}", is_playing);
        }
        if counter == 7 {
            println!("Restart");
            let file2 = File::open("sound2.wav").unwrap();
            let source2 = rodio::Decoder::new(BufReader::new(file2)).unwrap();
            sink.append(source2);
            sink.play();
        }
        counter += 1;
    }
}

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

fn do_play2() {
    let (mut sink, mut stream, mut stream_handle) = play("sound.flac");

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
            stream = stream_new;
            stream_handle = stream_handle_new;
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

fn main() {
    do_play2();
    //let (sink, stream, stream_handle) = play("sound.flac");
    // let path ="sound.flac";
    // let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // let file = File::open(path).unwrap();
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // let sink = Sink::try_new(&stream_handle).unwrap();
    // sink.set_volume(0.1);
    // sink.append(source);
    //loop {}
}
