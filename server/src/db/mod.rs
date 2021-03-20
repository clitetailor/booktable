use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");
    let manager =
        ConnectionManager::<PgConnection>::new(database_url);

    return r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
}
