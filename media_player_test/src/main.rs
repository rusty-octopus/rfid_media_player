use rodio::Decoder;
use rodio::Sink;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    //let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open("sound.flac").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    //let (sink, output_queue) = Sink::new_idle();
    sink.set_volume(1.0);
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
        if counter > 5 {
            sink.stop();
            let is_playing = !sink.empty();
            println!("Still playing? {}", is_playing);
        }
        if counter == 7 {
            println!("Restart");
            let file2 = File::open("sound.flac").unwrap();
            let source2 = rodio::Decoder::new(BufReader::new(file2)).unwrap();
            sink.append(source2);
            //sink.play();
        }
        counter += 1;
    }
}
