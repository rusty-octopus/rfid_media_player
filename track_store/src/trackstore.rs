#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

use crate::error::Error;
use crate::id::Id;
use crate::trackpath::TrackPath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait TrackStore {
    fn get_path(&self, id: &Id) -> Option<&TrackPath>;
}

pub fn load(yaml_string: &str) -> Result<impl TrackStore, Error> {
    let track_store_result: serde_yaml::Result<TrackStoreImplementation> =
        serde_yaml::from_str(yaml_string);
    match track_store_result {
        Err(error) => Err(Error::LoadError(error.to_string())),
        Ok(track_store) => Ok(track_store),
    }
}

#[derive(Serialize, Deserialize)]
struct TrackStoreImplementation(HashMap<Id, TrackPath>);

impl TrackStore for TrackStoreImplementation {
    fn get_path(&self, id: &Id) -> Option<&TrackPath> {
        self.0.get(id)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;

    #[test]
    fn test_serialize() {
        let mut map = HashMap::new();
        let id = Id::from(String::from("1234"));
        let path = TrackPath::from("path/to/track".to_string());
        map.insert(id, path);
        // let id = Id::from(String::from("4321"));
        // let path = TrackPath::from("path/to/track_2".to_string());
        // map.insert(id, path);
        let track_store = TrackStoreImplementation(map);
        let yaml = serde_yaml::to_string(&track_store).unwrap();
        let expected_string = "---
\"1234\": path/to/track";
        assert_eq!(expected_string, yaml);
    }

    #[test]
    fn test_deserialize() {
        let yaml = "1234: path/to/track
4321: path/to/track_2";
        let track_store = load(yaml).unwrap();
        let id = String::from("1234").into();
        let track_path = track_store.get_path(&id);
        assert!(track_path.is_some());
        let expected_path = String::from("path/to/track").into();
        assert_eq!(Some(&expected_path), track_path);

        let id = String::from("4321").into();
        let track_path = track_store.get_path(&id);
        assert!(track_path.is_some());
        let expected_path = String::from("path/to/track_2").into();
        assert_eq!(Some(&expected_path), track_path);

        let id = String::from("4").into();
        let track_path = track_store.get_path(&id);
        assert!(track_path.is_none());
    }

    #[test]
    fn test_load_error() {
        let yaml = "error";
        let load_result = load(yaml);
        assert!(load_result.is_err());
        match load_result {
            Err(error) => assert_eq!(
                Error::LoadError(String::from(
                    "invalid type: string \"error\", expected a map at line 1 column 1"
                )),
                error
            ),
            Ok(value) => assert!(false, "Expected error found value"),
        }
    }
}
