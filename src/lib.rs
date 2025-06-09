use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};

#[derive(Serialize, Deserialize)]
struct Secret {
    id: u32,
    key: String,
    value: String
}

pub struct Keybearer {
    connection: Connection
}

impl Keybearer {
    pub fn new(database_url: &str) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(database_url)?;
    }
}