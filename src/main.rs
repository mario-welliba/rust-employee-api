#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

use std::env;

mod db;
mod employees;
mod error;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT").expect("Please set port in .env");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //let mut listenfd = ListenFd::from_env();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(employees::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
