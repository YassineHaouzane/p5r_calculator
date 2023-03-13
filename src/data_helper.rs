use std::collections::HashMap;
use std::fs;

use actix_web::web;

use crate::data_definitions::*;
use crate::royal_data::*;

fn get_result_arcana(a: Arcana, b: Arcana) -> Option<Arcana> {
    ARCANA_2_COMBOS_ROYAL
        .into_iter()
        .find(|x| {
            let (s1, s2) = x.source;
            s1 == a && s2 == b
        })
        .map(|x| x.result)
}

fn fuse_normal(
    persona1: &Persona,
    persona2: &Persona,
    personas_by_arcana: &HashMap<Arcana, Vec<Persona>>,
) -> Option<Persona> {
    let level = 1 + ((persona1.level + persona2.level) / 2);
    if let Some(arcana) = get_result_arcana(persona1.arcana, persona2.arcana) {
        let personas_arcana = personas_by_arcana.get(&arcana).unwrap();
        if persona1.arcana == persona2.arcana {
            for persona in personas_arcana.into_iter().rev() {
                if persona.level <= level
                    && !(persona.special.is_some()
                        || persona.rare.is_some()
                        || persona.name == persona1.name
                        || persona.name == persona2.name)
                {
                    return Some(persona.clone());
                }
            }
        } else {
            for persona in personas_arcana.into_iter() {
                if persona.level >= level && persona.rare.is_none() && persona.rare.is_none() {
                    return Some(persona.clone());
                }
            }
        }
    }
    None
}

fn get_arcana_recipe(
    personas1: &Vec<Persona>,
    personas2: &Vec<Persona>,
    personas_by_arcana: &HashMap<Arcana, Vec<Persona>>,
    recipes: &mut Vec<Recipe>,
) -> () {
    for (k, persona1) in personas1.iter().enumerate() {
        for (j, persona2) in personas2.iter().enumerate() {
            if !((persona1.rare.is_some() && persona2.rare.is_none())
                || (persona2.rare.is_some() && persona1.rare.is_none()))
            {
                if !(persona1.arcana == persona2.arcana && k <= j) {
                    let result_o = fuse_normal(persona1, persona2, &personas_by_arcana);

                    if let Some(result) = result_o {
                        recipes.push(Recipe {
                            sources: vec![persona1.clone(), persona2.clone()],
                            result,
                        });
                    }
                }
            }
        }
    }
}

fn get_arcanas_recipes(
    arcana: Arcana,
    global_personas: &Vec<Persona>,
    personas_by_arcana: &HashMap<Arcana, Vec<Persona>>,
) -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = vec![];
    let arcana_combos: Vec<ArcanaCombo> = ARCANA_2_COMBOS_ROYAL
        .into_iter()
        .filter(|x| x.result == arcana)
        .collect();
    arcana_combos.into_iter().for_each(|x| {
        // If the arcana doesn't exist we have a problem...
        let personas1_o = personas_by_arcana.get(&x.source.0);
        let personas2_o = personas_by_arcana.get(&x.source.1);
        match (personas1_o, personas2_o) {
            (Some(personas1), Some(personas2)) => {
                get_arcana_recipe(personas1, personas2, &personas_by_arcana, &mut recipes)
            }
            x => println!("Could not find arcanas {:?}", x),
        }
    });

    RARE_PERSONAS
        .into_iter()
        .fold(0, |rare_index, rare_persona_name| {
            let rare_persona =
                find_persona(rare_persona_name.to_string(), global_personas).unwrap();
            let arcana_personas = personas_by_arcana.get(&arcana).unwrap();
            arcana_personas.into_iter().for_each(|normal_persona| {
                if rare_persona.name != normal_persona.name {
                    let resulting_persona =
                        rare_fuse(normal_persona, personas_by_arcana, rare_index);
                    resulting_persona.into_iter().for_each(|result| {
                        recipes.push(Recipe {
                            sources: vec![normal_persona.clone(), rare_persona.clone()],
                            result: result.clone(),
                        })
                    });
                }
            });
            rare_index + 1
        });

    return recipes;
}

fn get_new_persona_from_rare_modifier(
    main_persona_index: i32,
    rare_modifier: i32,
    personas_main_arcana: &Vec<Persona>,
) -> Option<Persona> {
    let new_index = main_persona_index + rare_modifier;
    let new_persona_o = if new_index >= 0 && new_index < (personas_main_arcana.len() as i32) {
        // Casting is fine here because of the former check
        personas_main_arcana
            .get(new_index as usize)
            .map(|p| p.clone())
    } else {
        None
    };
    new_persona_o
}

fn rare_fuse(
    normal_persona: &Persona,
    personas_by_arcana: &HashMap<Arcana, Vec<Persona>>,
    rare_index: usize,
) -> Option<Persona> {
    // All arcanas exist in this slice
    let (_, rare_modifiers) = RARE_COMBOS
        .into_iter()
        .find(|(arcana, _)| arcana == &normal_persona.arcana)
        .unwrap();
    let mut rare_modifier = rare_modifiers[rare_index];
    // All arcanas exist in this vector
    let personas_main_arcana = personas_by_arcana.get(&normal_persona.arcana).unwrap();
    // The persona should exist in it's arcana array
    let main_persona_index = personas_main_arcana
        .iter()
        .position(|x| x.name == normal_persona.name)
        .unwrap() as i32;
    let mut new_persona_o =
        get_new_persona_from_rare_modifier(main_persona_index, rare_modifier, personas_main_arcana);

    // Couldn't come up with a better way to do this
    loop {
        match new_persona_o {
            Some(new_persona) if new_persona.special.is_some() || new_persona.rare.is_some() => {
                if rare_modifier > 0 {
                    rare_modifier = rare_modifier + 1;
                } else if rare_modifier < 0 {
                    rare_modifier = rare_modifier - 1;
                }
                new_persona_o = get_new_persona_from_rare_modifier(
                    main_persona_index,
                    rare_modifier,
                    personas_main_arcana,
                )
            }
            _ => break,
        }
    }
    return new_persona_o;
}

fn is_good_recipe(recipe: &Recipe, persona: &Persona) -> bool {
    let sources = &recipe.sources;
    if sources[0].name == persona.name || sources[1].name == persona.name {
        false
    } else {
        recipe.result.name == persona.name
    }
}

pub fn get_recipes(
    persona: &Persona,
    global_personas: &Vec<Persona>,
    personas_by_arcana: &HashMap<Arcana, Vec<Persona>>,
    special_combos: &Vec<Recipe>,
) -> Vec<Recipe> {
    if persona.rare.is_some() {
        return vec![];
    }

    if persona.special.is_some() {
        let special_recipe = get_special_recipe(persona, special_combos);
        return special_recipe.map_or(vec![], |recipe| vec![recipe]);
    }

    let recipes = get_arcanas_recipes(persona.arcana, global_personas, personas_by_arcana);
    let filtered_recipes: Vec<Recipe> = recipes
        .into_iter()
        .filter(|recipe| is_good_recipe(recipe, &persona))
        .collect();
    filtered_recipes
}

fn get_special_recipe(persona: &Persona, special_combos: &Vec<Recipe>) -> Option<Recipe> {
    if persona.special.is_none() {
        eprintln!(
            "Asking special recipe for non special persona: {}",
            persona.name
        );
        return None;
    }

    special_combos
        .iter()
        .find(|&recipe| recipe.result.name == persona.name)
        // Cloning to avoid shared reference
        .map(|recipe| recipe.clone())
}

// Builders
pub fn build_global_data() -> web::Data<GlobalAppData> {
    let path = "./data/P5R_data.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json_data: PersonaGameFusionData = serde_json::from_str(&data).unwrap();
    let mut personas = json_data.personas;
    let special_combos: Vec<Recipe> = json_data
        .special_combos
        .into_iter()
        .map(|named_recipe| named_recipe.named_recipe_to_recipe(&personas))
        .collect();
    personas.sort_by(|a, b| {
        if a.level == b.level {
            a.name.cmp(&b.name)
        } else {
            a.level.cmp(&b.level)
        }
    });
    let personas_by_arcana = build_persona_by_arcana(&personas);
    let global_data = GlobalAppData {
        personas,
        personas_by_arcana,
        special_combos,
    };
    web::Data::new(global_data)
}

pub fn build_persona_by_arcana(personas: &Vec<Persona>) -> HashMap<Arcana, Vec<Persona>> {
    let mut acc: HashMap<Arcana, Vec<Persona>> = HashMap::new();
    personas.iter().for_each(|value| {
        let cloned_persona = value.clone();
        if let Some(arcana_list) = acc.get_mut(&value.arcana) {
            arcana_list.push(cloned_persona);
        } else {
            acc.insert(cloned_persona.arcana, vec![cloned_persona]);
        }
    });
    acc
}

#[cfg(test)]
mod tests {
    use super::{build_global_data, get_recipes};

    #[test]
    fn build_global_data_test() {
        let x = build_global_data();
        assert_eq!(x.personas.len(), 232);
        assert_eq!(
            x.personas_by_arcana
                .get(&crate::data_definitions::Arcana::Fool)
                .unwrap()
                .len(),
            17,
        );
    }
    #[test]
    fn check_king_frost() {
        let global_data = build_global_data();
        let king_frost = global_data
            .personas
            .iter()
            .find(|x| x.name == "King Frost")
            .unwrap();
        let recipes = get_recipes(
            &king_frost,
            &global_data.personas,
            &global_data.personas_by_arcana,
            &global_data.special_combos,
        );
        assert_eq!(recipes.len(), 174);
    }
}
