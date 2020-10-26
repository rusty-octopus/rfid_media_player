use crate::keymap::{KeyMap,Key};
use crate::keymaperror::KeyMapError;

struct NeuftechKeyMap{}

impl KeyMap for NeuftechKeyMap {
  fn map(key: u8) -> Result<Key,KeyMapError> {
    const OFFSET_KEY_TO_UTF8:u8 = 0x13;
    match key {
      // Key board codes are 30-30 for keys 1-9 (utf8 hex values 0x31-0x39)
      30..=38 => Ok(Key::Digit(char::from(key+OFFSET_KEY_TO_UTF8))),
      // Key board code 39 is for key 0
      39 => Ok(Key::Digit(char::from(0x30))),
      // Key board code for 40 is Enter
      40 => Ok(Key::Enter),
      // Neuftech should only report numbers, enter or control codes
      _ => Err(KeyMapError::from_key_not_existing(key))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::convert::TryInto;

  #[test]
  fn test_digits() {
    let digits: Vec<Result<Key, KeyMapError>> = ('1'..='9').map(|c|Ok(Key::Digit(c))).collect();
    let mapped_keys: Vec<Result<Key, KeyMapError>>= (30..39).map(|i|NeuftechKeyMap::map(usize::try_into(i).unwrap())).collect();
    assert_eq!(digits, mapped_keys);
    assert_eq!(Ok(Key::Digit('0')), NeuftechKeyMap::map(39));
  }

  #[test]
  fn test_enter() {
    assert_eq!(Ok(Key::Enter), NeuftechKeyMap::map(40));
  }

  #[test]
  fn test_error(){
    assert_eq!(Err(KeyMapError::from_key_not_existing(17)), NeuftechKeyMap::map(17));
  }
}