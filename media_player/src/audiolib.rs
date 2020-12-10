use crate::error::Error;
use crate::track::Track;

pub(crate) trait AudioLib {
    fn play(&self, track: Track) -> Result<(), Error>;
    fn stop(&self);
    fn is_playing(&self) -> bool;
}
