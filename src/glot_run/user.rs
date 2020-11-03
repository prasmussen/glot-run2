use std::time;
use uuid;

use crate::glot_run::util;



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub token: String,
    pub created: String,
    pub modified: String,
}


pub fn new(token: &str) -> User {
    let id = uuid::Uuid::new_v4();
    let now = time::SystemTime::now();

    User{
        id: id,
        token: token.to_string(),
        created: util::rfc3339(now),
        modified: util::rfc3339(now),
    }
}

pub fn update_token(user: &User, token: &str) -> User {
    let now = time::SystemTime::now();

    User{
        token: token.to_string(),
        modified: util::rfc3339(now),
        ..user.clone()
    }
}
