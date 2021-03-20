use diesel::prelude::*;

use crate::db::schema::rooms;
use crate::db::Pool;
use crate::models::room::*;

pub fn get_rooms(
    pool: &Pool,
) -> Result<Vec<Room>, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let items = rooms::table.load::<Room>(&conn)?;

    Ok(items)
}

pub fn create_room(
    pool: &Pool,
    room: &InputRoom,
) -> Result<Room, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_room = NewRoom {
        name: &room.name,
        capacity: room.capacity,
    };

    let res = diesel::insert_into(rooms::table)
        .values(&new_room)
        .get_result(&conn)?;

    Ok(res)
}
