mod controller;
mod data_definitions;
mod data_helper;
mod royal_data;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use controller::{get_personas, get_recipes_of_persona};
use data_helper::build_global_data;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::env;

pub enum EnvType {
    Dev,
    Prod,
}

impl EnvType {
    pub fn build_envtype(env: &str) -> Result<EnvType, String> {
        match env {
            "PROD" => Ok(EnvType::Prod),
            "DEV" => Ok(EnvType::Dev),
            _ => Err(format!("Could not parse env value: {}", env)),
        }
    }
}

pub struct Config {
    pub env: EnvType,
    pub front_origin: String,
}

impl Config {
    pub fn build_config() -> Config {
        let front_origin = env::var("FRONT_URI");
        let env_type = env::var("ENV_TYPE");

        let config_r = match (front_origin, env_type) {
            (Ok(front_origin), Ok(env_type)) => {
                let env = EnvType::build_envtype(&env_type);
                match env {
                    Ok(env) => Ok(Config { env, front_origin }),
                    Err(s) => Err(s),
                }
            }
            (Err(_), Err(_)) => Err("Could not find both env variable".to_string()),
            (Err(_), _) => Err("Could not find FRONT_URI env variable".to_string()),
            (_, Err(_)) => Err("Could not find ENV_TYPE env variable".to_string()),
        };
        match config_r {
            Ok(config) => config,
            Err(msg) => {
                error!("{}", msg);
                panic!()
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    const PORT: u16 = 8080;
    info!("Starting server on port 8080");

    HttpServer::new(|| {
        let config = Config::build_config();
        let cors = Cors::default()
            .allowed_origin(&config.front_origin)
            .allowed_methods(vec!["GET"]);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(build_global_data())
            .service(get_personas)
            .service(get_recipes_of_persona)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
