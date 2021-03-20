use actix_web::{App, HttpServer};

use server::db;
use server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::room::add_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
