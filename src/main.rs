use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DB_URL must be set");
    let app = latin_website::LatinApp::new(8080);
    app.run(database_url).await
}
