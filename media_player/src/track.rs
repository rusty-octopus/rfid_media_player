use std::path::Path;
#[derive(Debug, PartialEq, Clone)]
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

impl AsRef<Path> for Track {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

// impl AsRef<String> for Track {
//     fn as_ref(&self) -> &String {
//         &self.0
//     }
// }
