use std::fmt::{Display, Formatter, Result};
use std::error::Error;
#[derive(Debug,PartialEq)]
pub(crate) enum KeyMapError {
  KeyNotExisting(String)
}

impl KeyMapError {
  pub(crate) fn from_key_not_existing(key: u8) -> KeyMapError {
    Self::KeyNotExisting(format!("There is no key char for u8 value \"{}\".", key))
  }
}

impl Display for KeyMapError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::KeyNotExisting(msg) => write!(f,"{}", msg)
    }
  }
}

impl Error for KeyMapError {}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_key_not_existing() {
    let error = KeyMapError::from_key_not_existing(17);
    let expected_error_string: String = format!("There is no key char for u8 value \"{}\".", 17);
    assert_eq!(KeyMapError::KeyNotExisting(expected_error_string.clone()), error);
    assert_eq!(format!("KeyNotExisting({:?})", expected_error_string), format!("{:?}", error));
    assert_eq!(expected_error_string, format!("{}", error));
  }
}