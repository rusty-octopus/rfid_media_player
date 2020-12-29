#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod media_player_integration_tests {
    use media_player::{open, MediaPlayer, Track};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_double_play() {
        let mut media_player = open().unwrap();
        let track = Track::from("tests/rand1.wav");
        media_player.play(&track).unwrap();
        assert!(true);

        sleep(Duration::from_millis(200));

        let track2 = Track::from("tests/rand2.wav");
        media_player.play(&track2).unwrap();
        assert!(true);
        sleep(Duration::from_millis(200));

        media_player.stop().unwrap();
        assert!(true);
    }

    #[test]
    fn test_play_until_the_end() {
        let mut media_player = open().unwrap();
        let track = Track::from("tests/rand1.wav");
        media_player.play(&track).unwrap();
        for i in 0..33 {
            std::thread::sleep(Duration::from_secs(1));
            println!("Time elapsed: {} s", i + 1);
        }
        assert!(true);
    }
}
