use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivityTimestamps {
    pub end: Option<u64>,
    pub start: Option<u64>,
}
