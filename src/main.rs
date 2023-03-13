mod controller;
mod data_definitions;
mod data_helper;
mod royal_data;
use actix_web::{App, HttpServer};
use controller::{get_personas, get_recipes_of_persona};
use data_helper::build_global_data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(build_global_data())
            .service(get_personas)
            .service(get_recipes_of_persona)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
