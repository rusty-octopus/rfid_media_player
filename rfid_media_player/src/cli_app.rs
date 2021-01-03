#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use clap::{App, Arg};
use std::num::ParseIntError;

/// Name of the Vendor ID command line option.
pub const VENDOR_ID_ARGUMENT_NAME: &str = "Vendor ID";
/// Name of the Product ID command line option.
pub const PRODUCT_ID_ARGUMENT_NAME: &str = "Product ID";
/// Name of the Tracks File command line option.
pub const TRACKS_FILE_ARGUMENT_NAME: &str = "Tracks File";
/// Name of the Timeout command line option.
pub const TIMEOUT_ARGUMENT_NAME: &str = "Timeout";
/// Name of the Log Level command line option.
pub const LOG_LEVEL_ARGUMENT_NAME: &str = "Log Level";
/// Name of the Console Log command line flag.
pub const CONSOLE_LOG_ARGUMENT_NAME: &str = "Console Log";

/// Creates the [`clap::App`](https://docs.rs/clap/2.33.3/clap/struct.App.html) of the rfid_media_player application.
pub fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("RFID Media Player")
        .version("1.0.0")
        .author("rusty-octopus <octopus@posteo.net>")
        .about("Reads RFID cards and plays the tracks that are associated to the RFID values of these cards.")
        .arg(
            Arg::with_name(VENDOR_ID_ARGUMENT_NAME)
                .short("v")
                .long("vendor_id")
                .value_name("VENDOR_ID")
                .help("The Vendor ID of the RFID card reader.\
                Must be a valid unsigned integer with at most 16 bits.")
                .required(true).validator(validate_id),
        )
        .arg(
          Arg::with_name(PRODUCT_ID_ARGUMENT_NAME)
              .short("p")
              .long("product_id")
              .value_name("PRODUCT_ID")
              .help("The Product ID of the RFID card reader.\
              Must be a valid unsigned integer with at most 16 bits.")
              .required(true).validator(validate_id),
      )
      .arg(
        Arg::with_name(TRACKS_FILE_ARGUMENT_NAME)
            .short("t")
            .long("tracks")
            .value_name("TRACKS_FILE")
            .help("The YAML file that contains the mapping from RFID value to the path of the associated track.\
            Mapping must be defined like:\n\
            \"<rfid value as string incl. leading zeros>: <path to track>\"")
            .required(true),

      )
      .arg(
        Arg::with_name(TIMEOUT_ARGUMENT_NAME)
            .short("o")
            .long("timeout")
            .value_name("TIMEOUT")
            .help("The timeout in milliseconds (ms) for the USB device interrupt read.\
            This means the time the read of the USB device should be blocking.\
            Aborting the application (e.g. via Ctrl+c) may be delayed up by this timeout value.\
            Do not use this value unless you know what you are doing.\
            Must be a valid unsigned integer with at most 64 bits in decimal radix.")
            .required(false).validator(validate_timeout)
            .default_value("1000"))
      .arg(
        Arg::with_name(LOG_LEVEL_ARGUMENT_NAME)
          .short("l")
          .long("log_level")
          .value_name("LOG_LEVEL")
          .help("The log level for the logger.")
          .possible_value("error")
          .possible_value("warn")
          .possible_value("info")
          .possible_value("debug")
          .possible_value("trace")
          .required(false).default_value("info"))
      .arg(Arg::with_name(CONSOLE_LOG_ARGUMENT_NAME).long("console_log").short("c").help("Log will be duplicated to the console (stdout)."))
}

/// Tries to convert a [`String`](std::string::String) into an [`u16`](std::u16).
///
/// Returns a [`ParseIntError`](std::num::ParseIntError) whenever the given [`String`](std::string::String) contains an invalid decimal or hexidecimal digit.
pub fn convert_to_id(id_string: String) -> Result<u16, ParseIntError> {
    let mut conversion_result = u16::from_str_radix(&id_string, 16);
    if conversion_result.is_err() {
        conversion_result = u16::from_str_radix(&id_string, 10);
    }
    conversion_result
}

fn validate_id(id_string: String) -> Result<(), String> {
    let conversion_result = convert_to_id(id_string);

    if conversion_result.is_err() {
        return Err("ID value must be a valid hexadecimal or decimal unsigned integer number with at least 16 bits.".to_string());
    }
    Ok(())
}

fn validate_timeout(timeout_string: String) -> Result<(), String> {
    let conversion_result = u64::from_str_radix(&timeout_string, 10);

    if conversion_result.is_err() {
        return Err(
            "Timeout value must be a valid decimal unsigned integer with at least 16 bits."
                .to_string(),
        );
    }

    Ok(())
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;

    #[test]
    fn test_convert_to_id() {
        let result = convert_to_id("FF".to_string());
        assert_eq!(Ok(255), result);
        let result = convert_to_id("G".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_id() {
        let result = validate_id("FF".to_string());
        assert_eq!(Ok(()), result);
        let result = validate_id("G".to_string());
        assert_eq!(Err("ID value must be a valid hexadecimal or decimal unsigned integer number with at least 16 bits.".to_string()), result);
    }

    #[test]
    fn test_validate_timeout() {
        let result = validate_timeout("1000".to_string());
        assert_eq!(Ok(()), result);
        let result = validate_timeout("G".to_string());
        assert_eq!(
            Err(
                "Timeout value must be a valid decimal unsigned integer with at least 16 bits."
                    .to_string()
            ),
            result
        );
    }

    #[test]
    fn test_create_app() {
        let app = create_app();
        let matches = app.get_matches_from(vec![
            "rfid_media_player",
            "--vendor_id",
            "1",
            "--product_id",
            "2",
            "--tracks",
            "path/to/tracks",
        ]);
        assert_eq!(Some("1"), matches.value_of(VENDOR_ID_ARGUMENT_NAME));
        assert_eq!(Some("2"), matches.value_of(PRODUCT_ID_ARGUMENT_NAME));
        assert_eq!(
            Some("path/to/tracks"),
            matches.value_of(TRACKS_FILE_ARGUMENT_NAME)
        );
    }
}
