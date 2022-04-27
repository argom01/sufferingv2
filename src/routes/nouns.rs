//use crate::errors::AppError;
//use crate::{models, Pool};
//
//use actix_web::{web, HttpResponse};
//
//async fn add_noun(
//    noun: web::Json<models::nouns::Noun>,
//    pool: web::Data<Pool>,
//) -> Result<HttpResponse, AppError> {
//    web::block(move || {
//        let conn = &mut pool.get().unwrap();
//        models::nouns::add_noun(conn, noun.into_inner())
//    })
//    .await?;
//
//    Ok(HttpResponse::Ok().json("Successfully added a noun"))
//}
//
//pub fn configure(cfg: &mut web::ServiceConfig) {
//    cfg.service(web::resource("/nouns").route(web::post().to(add_noun)));
//}
