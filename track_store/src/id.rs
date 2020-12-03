use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Id(String);

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id(s)
    }
}
