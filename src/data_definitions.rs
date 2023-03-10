use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Arcana {
    Fool,
    Magician,
    Priestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    Fortune,
    Strength,
    Hanged,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
    Councillor,
    Faith,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Persona {
    pub name: String,
    pub inherits: String,
    pub arcana: Arcana,
    pub rare: Option<bool>,
    pub special: Option<bool>,
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub sources: Vec<Persona>,
    pub result: Persona,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalAppData {
    pub personas: Vec<Persona>,
    pub personas_by_arcana: HashMap<Arcana, Vec<Persona>>,
}
