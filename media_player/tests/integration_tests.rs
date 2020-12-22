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

        sleep(Duration::from_millis(200));

        let track2 = Track::from("tests/rand2.wav");
        media_player.play(&track2).unwrap();

        sleep(Duration::from_millis(200));

        media_player.stop().unwrap();
    }
}
