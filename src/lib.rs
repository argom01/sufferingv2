#[macro_use]
extern crate serde_derive;

use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use jwt_simple::prelude::*;
use lazy_static::lazy_static;

mod auth;
mod errors;
mod models;
mod routes;

mod prisma;

use prisma::PrismaClient;

lazy_static! {
    static ref ACCESS_TOKEN_SECRET: HS256Key = HS256Key::generate();
    static ref REFRESH_TOKEN_SECRET: HS256Key = HS256Key::generate();
}

pub struct LatinApp {
    port: u16,
}

pub struct DbClient {
    client: Mutex<PrismaClient>,
}

impl LatinApp {
    pub fn new(port: u16) -> Self {
        LatinApp { port }
    }

    pub async fn run(&self, db_url: String) -> std::io::Result<()> {
        let client = web::Data::new(DbClient {
            client: Mutex::new(
                prisma::new_client()
                    .await
                    .expect("could not create prisma client"),
            ),
        });
        println!("Starting server at port {}", self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(client.clone())
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
            //.configure(routes::nouns::configure)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}
