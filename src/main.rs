use actix_web::{get, web, Responder, Result};
use actix_web::{App, HttpServer};
mod data_definitions;
use data_definitions::GlobalAppData;
mod data_helper;
use data_helper::{build_global_data, find_persona, get_recipes};
mod royal_data;

#[get("/")]
async fn greet(p: web::Data<GlobalAppData>) -> Result<impl Responder> {
    Ok(web::Json(p))
}
#[get("/{persona_name}")]
async fn get_recipes_of_persona(
    persona_name: web::Path<String>,
    p: web::Data<GlobalAppData>,
) -> Result<impl Responder> {
    let global_personas = &p.personas;
    let persona = find_persona(persona_name.to_string(), global_personas).unwrap();
    let recipes = get_recipes(persona, global_personas, &p.personas_by_arcana);
    Ok(web::Json(recipes))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(build_global_data())
            .service(greet)
            .service(get_recipes_of_persona)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
