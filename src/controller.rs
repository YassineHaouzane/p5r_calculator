use actix_web::{error, get, http::StatusCode, web, HttpRequest, Responder, Result};
use derive_more::{Display, Error};

use crate::{
    data_definitions::{find_persona, GlobalAppData},
    data_helper::get_recipes,
};

#[get("/personas")]
async fn get_personas(p: web::Data<GlobalAppData>, req: HttpRequest) -> Result<impl Responder> {
    Ok(web::Json(&p.personas).respond_to(&req))
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Persona not found: {}", persona_name)]
struct PersonaNotFound {
    persona_name: String,
}

impl error::ResponseError for PersonaNotFound {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}

#[get("/recipes/{persona_name}")]
async fn get_recipes_of_persona(
    persona_name: web::Path<String>,
    p: web::Data<GlobalAppData>,
) -> Result<impl Responder, PersonaNotFound> {
    let global_personas = &p.personas;
    let persona_o = find_persona(persona_name.to_string(), global_personas);
    if let Some(persona) = persona_o {
        let recipes = get_recipes(persona, p.get_ref());
        Ok(web::Json(recipes))
    } else {
        println!("Not found");
        Err(PersonaNotFound {
            persona_name: persona_name.into_inner(),
        })
    }
}
