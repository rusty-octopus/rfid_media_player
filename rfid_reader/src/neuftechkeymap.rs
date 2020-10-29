use crate::keymap::{Key, KeyMap};
use crate::keymaperror::KeyMapError;

pub struct NeuftechKeyMap;

impl KeyMap for NeuftechKeyMap {
    fn map(&self, key: u8) -> Result<Key, KeyMapError> {
        const OFFSET_KEY_TO_UTF8: u8 = 0x13;
        match key {
            // Key board codes are 30-30 for keys 1-9 (utf8 hex values 0x31-0x39)
            30..=38 => Ok(Key::Digit(char::from(key + OFFSET_KEY_TO_UTF8))),
            // Key board code 39 is for key 0
            39 => Ok(Key::Digit(char::from(0x30))),
            // Key board code for 40 is Enter
            40 => Ok(Key::Enter),
            // Neuftech should only report numbers, enter or control codes
            _ => Err(KeyMapError::from_key_not_existing(key)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_digits() {
        let keymap = NeuftechKeyMap;
        let digits: Vec<Result<Key, KeyMapError>> =
            ('1'..='9').map(|c| Ok(Key::Digit(c))).collect();
        let mapped_keys: Vec<Result<Key, KeyMapError>> = (30..39)
            .map(|i| keymap.map(usize::try_into(i).unwrap()))
            .collect();
        assert_eq!(digits, mapped_keys);
        assert_eq!(Ok(Key::Digit('0')), keymap.map(39));
    }

    #[test]
    fn test_enter() {
        let keymap = NeuftechKeyMap;
        assert_eq!(Ok(Key::Enter), keymap.map(40));
    }

    #[test]
    fn test_error() {
        let keymap = NeuftechKeyMap;
        assert_eq!(Err(KeyMapError::from_key_not_existing(17)), keymap.map(17));
    }
}
