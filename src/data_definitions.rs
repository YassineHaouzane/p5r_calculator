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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Persona {
    pub name: String,
    //pub inherits: String,
    pub arcana: Arcana,
    rare: Option<bool>,
    special: Option<bool>,
    pub level: u8,
}

impl Persona {
    pub fn is_special(&self) -> bool {
        self.special.is_some()
    }
    pub fn is_rare(&self) -> bool {
        self.rare.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    sources: Vec<Persona>,
    result: Persona,
    cost: u32,
}

impl Recipe {
    pub fn is_result(&self, persona: &Persona) -> bool {
        &self.result == persona
    }

    pub fn is_good_recipe(&self, persona: &Persona) -> bool {
        let sources = &self.sources;
        if sources[0].name == persona.name || sources[1].name == persona.name {
            false
        } else {
            &self.result == persona
        }
    }

    fn approx_cost(sources: &[Persona]) -> u32 {
        sources.iter().fold(0, |acc, p| {
            let level = p.level as u32;
            acc + (27 * level.pow(2)) + (126 * level) + 2147
        })
    }

    pub fn new(sources: Vec<Persona>, result: Persona) -> Recipe {
        let cost = Self::approx_cost(&sources);
        Recipe {
            sources,
            result,
            cost,
        }
    }

    // For testing purposes
    #[allow(dead_code)]
    pub fn recipe_len(&self) -> usize {
        self.sources.len()
    }
}

#[derive(Serialize, Deserialize)]
pub struct GlobalAppData {
    pub personas: Vec<Persona>,
    pub personas_by_arcana: HashMap<Arcana, Vec<Persona>>,
    pub special_combos: Vec<Recipe>,
}

// Not in data helper to avoid import cycle
pub fn find_persona(persona_name: String, global_personas: &[Persona]) -> Option<&Persona> {
    global_personas.iter().find(|x| x.name == persona_name)
}

// Struct to handle JSON structure of a persona game fusion data
#[derive(Serialize, Deserialize)]
pub struct PersonaGameFusionData {
    pub personas: Vec<Persona>,
    pub special_combos: Vec<NamedRecipe>,
}

// Used for special recipes (JSON Data contains only persona's names)
#[derive(Serialize, Deserialize)]
pub struct NamedRecipe {
    result: String,
    sources: Vec<String>,
}

impl NamedRecipe {
    pub fn named_recipe_to_recipe(self, personas: &[Persona]) -> Recipe {
        //unwraping because all personas should exist
        let result = find_persona(self.result, personas).unwrap().clone();
        let sources: Vec<Persona> = self
            .sources
            .into_iter()
            .map(|name| find_persona(name, personas).unwrap().clone())
            .collect();
        Recipe::new(sources, result)
    }
}
