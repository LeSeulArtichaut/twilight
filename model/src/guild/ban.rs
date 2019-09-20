use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ban {
    pub reason: Option<String>,
    pub user: User,
}
