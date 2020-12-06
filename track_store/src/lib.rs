//! Crate that stores paths to tracks and enables access to these paths by using ids.
//!
//! The [`TrackStore`](crate::TrackStore) can be [`load`](crate::load)ed from a yaml string that is a list
//! of key value pairs.
//! Each key is an [`Id`](crate::Id) defined by a [`String`](std::string::String).
//! Each value is a [`TrackPath`](crate::TrackPath) defined by a [`String`](std::string::String).
//!
//! # Example
//! ```rust
//! // use crate
//! use track_store::{load, TrackStore, Id, TrackPath};
//!
//! // simple key value list in the yaml string
//! let yaml_string = "01234: path/to/track";
//!
//! // load the TrackStore from the yaml string.
//! let track_store = load(yaml_string).unwrap();
//!
//! // Create an Id from a string
//! let id = Id::from(String::from("01234"));
//!
//! // get the track path as an Option
//! let track_path = track_store.get_path(&id);
//! assert!(track_path.is_some());
//! let expected_path = String::from("path/to/track").into();
//! assert_eq!(Some(&expected_path), track_path);
//!
//! // If the Id does not exist, the returned track path is None.
//! let id = Id::from(String::from("0"));
//! let track_path = track_store.get_path(&id);
//! assert!(track_path.is_none());
//! ```
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

mod error;
mod id;
mod trackpath;
mod trackstore;

pub use error::Error;
pub use id::Id;
pub use trackpath::TrackPath;
pub use trackstore::{load, TrackStore};
