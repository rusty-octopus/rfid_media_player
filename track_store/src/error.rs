#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

/// Track store errors errors.
///
/// Defines all runtime errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Load error.
    ///
    /// Returned when [`load`](crate::load)ing the [`TrackStore`](crate::TrackStore) did not work,
    /// e.g. when the parsed yaml string is invalid.
    LoadError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        let error = Error::LoadError(String::from("test"));
        let formatted_string = format!("{}", error);
        assert_eq!("LoadError(\"test\")", formatted_string);
    }
}
