use crate::id::Id;
use crate::trackpath::TrackPath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct TrackStoreImplementation(HashMap<Id, TrackPath>);

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
        let id = Id::from(String::from("4321"));
        let path = TrackPath::from("path/to/track_2".to_string());
        map.insert(id, path);
        let track_store = TrackStoreImplementation(map);
        let yaml = serde_yaml::to_string(&track_store).unwrap();
        assert_eq!("", yaml);
    }
}
