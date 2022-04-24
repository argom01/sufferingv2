#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use jwt_simple::prelude::*;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

mod auth;
mod errors;
mod models;
mod routes;
mod schema;

lazy_static! {
    static ref ACCESS_TOKEN_SECRET: HS256Key = HS256Key::generate();
    static ref REFRESH_TOKEN_SECRET: HS256Key = HS256Key::generate();
}

pub struct LatinApp {
    port: u16,
}

impl LatinApp {
    pub fn new(port: u16) -> Self {
        LatinApp { port }
    }

    pub async fn run(&self, db_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        println!("Starting server at port {}", self.port);
        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
                .configure(routes::nouns::configure)
        })
        .bind(("0.0.0.0", self.port))?
        .run()
        .await
    }
}
