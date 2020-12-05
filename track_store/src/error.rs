#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
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
