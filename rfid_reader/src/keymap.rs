#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub(crate) enum Key {
    Digit(char),
    Enter,
}
pub(crate) trait KeyMap: std::fmt::Debug {
    fn map(&self, key: u8) -> Result<Key, Error>;
}

impl Into<char> for Key {
    fn into(self) -> char {
        match self {
            Self::Digit(c) => c,
            Self::Enter => 'Z',
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_into_char() {
        let digit = Key::Digit('9');
        let c = digit.into();
        assert_eq!('9', c);
        let digit = Key::Enter;
        let c = digit.into();
        assert_eq!('Z', c);
    }
}
