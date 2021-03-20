use actix_web::{get, post, web, HttpResponse, Responder};

use crate::db::Pool;
use crate::models::room::*;
use crate::services::room as room_service;

pub fn add_route(cfg: &mut web::ServiceConfig) {
    cfg.service(get_rooms).service(create_room);
}

#[get("/rooms")]
async fn get_rooms(pool: web::Data<Pool>) -> impl Responder {
    let rooms = room_service::get_rooms(&pool).unwrap();

    HttpResponse::Ok().json(rooms)
}

#[post("/rooms")]
async fn create_room(
    pool: web::Data<Pool>,
    room: web::Json<InputRoom>,
) -> impl Responder {
    let room =
        room_service::create_room(&pool, &room.into_inner())
            .unwrap();

    HttpResponse::Created().json(room)
}
