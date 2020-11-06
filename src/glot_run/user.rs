use std::time;
use uuid;

use crate::glot_run::util;



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub token: ascii::AsciiString,
    pub created: String,
    pub modified: String,
}


pub fn new(token: &ascii::AsciiString) -> User {
    let id = uuid::Uuid::new_v4();
    let now = time::SystemTime::now();

    User{
        id: id,
        token: token.clone(),
        created: util::rfc3339(now),
        modified: util::rfc3339(now),
    }
}

pub fn update_token(user: &User, token: &ascii::AsciiString) -> User {
    let now = time::SystemTime::now();

    User{
        token: token.clone(),
        modified: util::rfc3339(now),
        ..user.clone()
    }
}
