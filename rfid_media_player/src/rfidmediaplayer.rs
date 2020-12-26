use crate::logging::Logging;

use media_player::MediaPlayer;
use rfid_reader::RfidReader;
use track_store::TrackStore;
pub(crate) trait RfidMediaPlayer {
    fn run();
    fn shutdown();
}

struct RfidMediaPlayerImplementation<L, M, R, T>
where
    L: Logging,
    M: MediaPlayer,
    R: RfidReader,
    T: TrackStore,
{
    logging: L,
    media_player: M,
    rfid_reader: R,
    track_store: T,
}

pub(crate) fn open<L, M, R, T>(logging: L, media_player: M, rfid_reader: R, track_store: T)
where
    L: Logging,
    M: MediaPlayer,
    R: RfidReader,
    T: TrackStore,
{
    todo!();
}
