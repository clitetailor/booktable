use serde::{Deserialize, Serialize};

use crate::db::schema::rooms;

#[derive(Debug, Deserialize)]
pub struct InputRoom {
    pub name: String,
    pub capacity: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "rooms"]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub capacity: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
}
