#[derive(Debug, PartialEq)]
pub struct Track(String);

impl From<String> for Track {
    fn from(path: String) -> Self {
        Track(path)
    }
}

impl From<&str> for Track {
    fn from(path: &str) -> Self {
        Track(path.into())
    }
}
