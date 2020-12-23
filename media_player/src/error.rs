#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

/// Media player errors.
///
/// Defines all runtime errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Audio library  error.
    ///
    /// Returned whenever there was an error raised by the underlying audio library.
    AudioLibError(String),
    /// IO error.
    ///
    /// Returned whenever there was a file access error.
    IoError(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(format!("{}", error))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = Error::AudioLibError(String::from("test"));
        let formatted_string = format!("{}", error);
        assert_eq!("AudioLibError(\"test\")", formatted_string);
    }

    #[test]
    fn test_from_io_error() {
        let result = std::fs::File::open("non_existing_file");
        let io_error = result.unwrap_err();
        let error = Error::from(io_error);
        assert_eq!(
            Error::IoError(String::from("No such file or directory (os error 2)")),
            error
        );
    }
}
