use crate::keymaperror::KeyMapError;

#[derive(Debug,PartialEq)]
pub(crate) enum Key {
  Digit(char),
  Enter,
}
pub(crate) trait KeyMap {
  fn map(&self, key:u8) -> Result<Key,KeyMapError>;
}