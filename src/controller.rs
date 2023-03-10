use actix_web::{get, web, HttpRequest, Responder, Result};

use crate::{
    data_definitions::{find_persona, GlobalAppData},
    data_helper::get_recipes,
};

#[get("/personas")]
async fn get_personas(p: web::Data<GlobalAppData>, req: HttpRequest) -> Result<impl Responder> {
    Ok(web::Json(&p.personas).respond_to(&req))
}

#[get("/{persona_name}")]
async fn get_recipes_of_persona(
    persona_name: web::Path<String>,
    p: web::Data<GlobalAppData>,
) -> Result<impl Responder> {
    let global_personas = &p.personas;
    let persona = find_persona(persona_name.to_string(), global_personas).unwrap();
    let recipes = get_recipes(persona, p.get_ref());
    Ok(web::Json(recipes))
}
