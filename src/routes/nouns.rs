use crate::errors::AppError;
use crate::prisma::PrismaClient;
use crate::{models, prisma::noun};

use actix_web::{web, HttpResponse};

async fn add_noun(
    noun: web::Json<noun::Data>,
    conn: web::Data<PrismaClient>,
) -> Result<HttpResponse, AppError> {
    models::nouns::add_noun(&conn, noun.into_inner()).await?;

    Ok(HttpResponse::Ok().json("Successfully added a noun"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/nouns").route(web::post().to(add_noun)));
}
