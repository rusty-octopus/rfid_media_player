use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub(crate) struct TrackPath(String);

impl From<String> for TrackPath {
    fn from(s: String) -> Self {
        TrackPath(s)
    }
}
