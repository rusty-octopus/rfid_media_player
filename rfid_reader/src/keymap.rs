use crate::keymaperror::KeyMapError;

#[derive(Debug, PartialEq)]
pub(crate) enum Key {
    Digit(char),
    Enter,
}
pub(crate) trait KeyMap {
    fn map(&self, key: u8) -> Result<Key, KeyMapError>;
}

impl Into<char> for Key {
    fn into(self) -> char {
        match self {
            Self::Digit(c) => c,
            Self::Enter => 'Z',
        }
    }
}