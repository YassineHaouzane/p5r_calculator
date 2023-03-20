mod controller;
mod data_definitions;
mod data_helper;
mod royal_data;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use controller::{get_personas, get_recipes_of_persona};
use data_helper::build_global_data;
use env_logger::Env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server on port 8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET"]);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(build_global_data())
            .service(get_personas)
            .service(get_recipes_of_persona)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
