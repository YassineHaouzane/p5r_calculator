use serde::{Deserialize, Serialize};

pub const RARE_PERSONAS: [&str; 9] = [
    "Crystal Skull",
    "Koh-i-Noor",
    "Queen's Necklace",
    "Regent",
    "Stone of Scone",
    "Orlov",
    "Emperor's Amulet",
    "Hope Diamond",
    "Orichalcum",
];

pub const RARE_COMBOS: [(Arcana, [i32; 9]); 24] = [
    (Fool, [2, 1, 1, -1, -1, -1, -1, 1, 1]),
    (Magician, [1, -1, -1, 1, 2, 1, 1, 1, -2]),
    (Priestess, [1, 2, 1, -1, -1, 1, -1, 1, 1]),
    (Empress, [1, 1, 2, -1, -1, 1, 1, -1, 1]),
    (Emperor, [-1, -1, -1, 2, 1, -1, 1, 1, -1]),
    (Hierophant, [-1, -2, -1, 1, 1, -1, -1, -1, 2]),
    (Lovers, [1, 1, 1, -1, -1, -1, -1, -2, -1]),
    (Chariot, [-1, -1, -1, 1, 1, 1, -2, -1, -1]),
    (Justice, [1, 2, -1, 1, -1, -2, 1, -1, 1]),
    (Hermit, [-2, 1, -1, 2, -1, -1, 1, 1, -1]),
    (Fortune, [-1, -1, -1, 1, 2, 1, -1, 1, 1]),
    (Strength, [-1, 1, 1, -1, 1, 2, 1, -1, -1]),
    (Hanged, [-1, -1, 1, 1, -1, 1, 2, -1, 1]),
    (Death, [1, 1, -1, 1, 1, -1, -1, 2, 1]),
    (Temperance, [2, -1, 1, -1, -1, 1, 1, -1, -1]),
    (Devil, [-1, 1, 1, -2, -1, -1, 2, -1, 1]),
    (Tower, [-1, 1, -2, -1, 1, -1, 1, -1, -1]),
    (Star, [1, -1, -1, 1, 1, 2, -1, 1, 1]),
    (Moon, [-1, -1, 2, -1, -2, 1, -1, -1, -1]),
    (Sun, [1, 1, -1, 1, -1, 1, -1, 2, 1]),
    (Judgement, [-1, -1, -1, -1, 1, -1, 1, -1, 1]),
    (Faith, [1, 1, 1, -1, 1, -1, 1, 1, 2]),
    (Councillor, [1, 1, 1, 1, 1, -1, 1, 1, 1]),
    (World, [1, 1, 1, 1, 1, 1, 1, 1, 1]),
];

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

use Arcana::*;

pub struct ArcanaCombo {
    pub source: (Arcana, Arcana),
    pub result: Arcana,
}

pub const ARCANA_2_COMBOS_ROYAL: [ArcanaCombo; 273] = [
    ArcanaCombo {
        source: (Fool, Fool),
        result: Fool,
    },
    ArcanaCombo {
        source: (Fool, Magician),
        result: Death,
    },
    ArcanaCombo {
        source: (Fool, Priestess),
        result: Moon,
    },
    ArcanaCombo {
        source: (Fool, Empress),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Fool, Emperor),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Fool, Hierophant),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Fool, Lovers),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Fool, Chariot),
        result: Moon,
    },
    ArcanaCombo {
        source: (Fool, Justice),
        result: Star,
    },
    ArcanaCombo {
        source: (Fool, Hermit),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Fool, Fortune),
        result: Faith,
    },
    ArcanaCombo {
        source: (Fool, Strength),
        result: Death,
    },
    ArcanaCombo {
        source: (Fool, Hanged),
        result: Tower,
    },
    ArcanaCombo {
        source: (Fool, Death),
        result: Strength,
    },
    ArcanaCombo {
        source: (Fool, Temperance),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Fool, Devil),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Fool, Tower),
        result: Empress,
    },
    ArcanaCombo {
        source: (Fool, Star),
        result: Magician,
    },
    ArcanaCombo {
        source: (Fool, Moon),
        result: Justice,
    },
    ArcanaCombo {
        source: (Fool, Sun),
        result: Justice,
    },
    ArcanaCombo {
        source: (Fool, Judgement),
        result: Sun,
    },
    ArcanaCombo {
        source: (Fool, Faith),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Fool, Councillor),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Magician, Magician),
        result: Magician,
    },
    ArcanaCombo {
        source: (Magician, Priestess),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Magician, Empress),
        result: Justice,
    },
    ArcanaCombo {
        source: (Magician, Emperor),
        result: Faith,
    },
    ArcanaCombo {
        source: (Magician, Hierophant),
        result: Death,
    },
    ArcanaCombo {
        source: (Magician, Lovers),
        result: Devil,
    },
    ArcanaCombo {
        source: (Magician, Chariot),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Magician, Justice),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Magician, Hermit),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Magician, Fortune),
        result: Justice,
    },
    ArcanaCombo {
        source: (Magician, Strength),
        result: Fool,
    },
    ArcanaCombo {
        source: (Magician, Hanged),
        result: Empress,
    },
    ArcanaCombo {
        source: (Magician, Death),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Magician, Temperance),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Magician, Devil),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Magician, Tower),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Magician, Star),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Magician, Moon),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Magician, Sun),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Magician, Judgement),
        result: Strength,
    },
    ArcanaCombo {
        source: (Magician, Faith),
        result: Strength,
    },
    ArcanaCombo {
        source: (Magician, Councillor),
        result: Moon,
    },
    ArcanaCombo {
        source: (Priestess, Priestess),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Priestess, Empress),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Priestess, Emperor),
        result: Empress,
    },
    ArcanaCombo {
        source: (Priestess, Hierophant),
        result: Magician,
    },
    ArcanaCombo {
        source: (Priestess, Lovers),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Priestess, Chariot),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Priestess, Justice),
        result: Death,
    },
    ArcanaCombo {
        source: (Priestess, Hermit),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Priestess, Fortune),
        result: Magician,
    },
    ArcanaCombo {
        source: (Priestess, Strength),
        result: Devil,
    },
    ArcanaCombo {
        source: (Priestess, Hanged),
        result: Death,
    },
    ArcanaCombo {
        source: (Priestess, Death),
        result: Magician,
    },
    ArcanaCombo {
        source: (Priestess, Temperance),
        result: Devil,
    },
    ArcanaCombo {
        source: (Priestess, Devil),
        result: Moon,
    },
    ArcanaCombo {
        source: (Priestess, Tower),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Priestess, Star),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Priestess, Moon),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Priestess, Sun),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Priestess, Judgement),
        result: Justice,
    },
    ArcanaCombo {
        source: (Priestess, Faith),
        result: Justice,
    },
    ArcanaCombo {
        source: (Priestess, Councillor),
        result: Faith,
    },
    ArcanaCombo {
        source: (Empress, Empress),
        result: Empress,
    },
    ArcanaCombo {
        source: (Empress, Emperor),
        result: Justice,
    },
    ArcanaCombo {
        source: (Empress, Hierophant),
        result: Fool,
    },
    ArcanaCombo {
        source: (Empress, Lovers),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Empress, Chariot),
        result: Star,
    },
    ArcanaCombo {
        source: (Empress, Justice),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Empress, Hermit),
        result: Strength,
    },
    ArcanaCombo {
        source: (Empress, Fortune),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Empress, Strength),
        result: Faith,
    },
    ArcanaCombo {
        source: (Empress, Hanged),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Empress, Death),
        result: Fool,
    },
    ArcanaCombo {
        source: (Empress, Temperance),
        result: Faith,
    },
    ArcanaCombo {
        source: (Empress, Devil),
        result: Sun,
    },
    ArcanaCombo {
        source: (Empress, Tower),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Empress, Star),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Empress, Moon),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Empress, Sun),
        result: Tower,
    },
    ArcanaCombo {
        source: (Empress, Judgement),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Empress, Faith),
        result: Magician,
    },
    ArcanaCombo {
        source: (Empress, Councillor),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Emperor, Emperor),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Emperor, Hierophant),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Emperor, Lovers),
        result: Fool,
    },
    ArcanaCombo {
        source: (Emperor, Chariot),
        result: Faith,
    },
    ArcanaCombo {
        source: (Emperor, Justice),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Emperor, Hermit),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Emperor, Fortune),
        result: Sun,
    },
    ArcanaCombo {
        source: (Emperor, Strength),
        result: Tower,
    },
    ArcanaCombo {
        source: (Emperor, Hanged),
        result: Devil,
    },
    ArcanaCombo {
        source: (Emperor, Death),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Emperor, Temperance),
        result: Devil,
    },
    ArcanaCombo {
        source: (Emperor, Devil),
        result: Justice,
    },
    ArcanaCombo {
        source: (Emperor, Tower),
        result: Star,
    },
    ArcanaCombo {
        source: (Emperor, Star),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Emperor, Moon),
        result: Tower,
    },
    ArcanaCombo {
        source: (Emperor, Sun),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Emperor, Judgement),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Emperor, Faith),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Emperor, Councillor),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Hierophant, Hierophant),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Hierophant, Lovers),
        result: Strength,
    },
    ArcanaCombo {
        source: (Hierophant, Chariot),
        result: Star,
    },
    ArcanaCombo {
        source: (Hierophant, Justice),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Hierophant, Hermit),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Hierophant, Fortune),
        result: Justice,
    },
    ArcanaCombo {
        source: (Hierophant, Strength),
        result: Fool,
    },
    ArcanaCombo {
        source: (Hierophant, Hanged),
        result: Sun,
    },
    ArcanaCombo {
        source: (Hierophant, Death),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Hierophant, Temperance),
        result: Death,
    },
    ArcanaCombo {
        source: (Hierophant, Devil),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Hierophant, Tower),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Hierophant, Star),
        result: Tower,
    },
    ArcanaCombo {
        source: (Hierophant, Moon),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Hierophant, Sun),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Hierophant, Judgement),
        result: Faith,
    },
    ArcanaCombo {
        source: (Hierophant, Faith),
        result: Empress,
    },
    ArcanaCombo {
        source: (Hierophant, Councillor),
        result: Justice,
    },
    ArcanaCombo {
        source: (Lovers, Lovers),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Lovers, Chariot),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Lovers, Justice),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Lovers, Hermit),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Lovers, Fortune),
        result: Strength,
    },
    ArcanaCombo {
        source: (Lovers, Strength),
        result: Death,
    },
    ArcanaCombo {
        source: (Lovers, Hanged),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Lovers, Death),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Lovers, Temperance),
        result: Strength,
    },
    ArcanaCombo {
        source: (Lovers, Devil),
        result: Moon,
    },
    ArcanaCombo {
        source: (Lovers, Tower),
        result: Empress,
    },
    ArcanaCombo {
        source: (Lovers, Star),
        result: Faith,
    },
    ArcanaCombo {
        source: (Lovers, Moon),
        result: Magician,
    },
    ArcanaCombo {
        source: (Lovers, Sun),
        result: Empress,
    },
    ArcanaCombo {
        source: (Lovers, Judgement),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Lovers, Faith),
        result: Tower,
    },
    ArcanaCombo {
        source: (Lovers, Councillor),
        result: Tower,
    },
    ArcanaCombo {
        source: (Chariot, Chariot),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Chariot, Justice),
        result: Moon,
    },
    ArcanaCombo {
        source: (Chariot, Hermit),
        result: Devil,
    },
    ArcanaCombo {
        source: (Chariot, Fortune),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Chariot, Strength),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Chariot, Hanged),
        result: Fool,
    },
    ArcanaCombo {
        source: (Chariot, Death),
        result: Devil,
    },
    ArcanaCombo {
        source: (Chariot, Temperance),
        result: Strength,
    },
    ArcanaCombo {
        source: (Chariot, Devil),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Chariot, Tower),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Chariot, Star),
        result: Moon,
    },
    ArcanaCombo {
        source: (Chariot, Moon),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Chariot, Sun),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Chariot, Faith),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Chariot, Councillor),
        result: Sun,
    },
    ArcanaCombo {
        source: (Justice, Justice),
        result: Justice,
    },
    ArcanaCombo {
        source: (Justice, Hermit),
        result: Magician,
    },
    ArcanaCombo {
        source: (Justice, Fortune),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Justice, Strength),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Justice, Hanged),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Justice, Death),
        result: Fool,
    },
    ArcanaCombo {
        source: (Justice, Temperance),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Justice, Devil),
        result: Fool,
    },
    ArcanaCombo {
        source: (Justice, Tower),
        result: Sun,
    },
    ArcanaCombo {
        source: (Justice, Star),
        result: Empress,
    },
    ArcanaCombo {
        source: (Justice, Moon),
        result: Devil,
    },
    ArcanaCombo {
        source: (Justice, Sun),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Justice, Faith),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Justice, Councillor),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Hermit, Hermit),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Hermit, Fortune),
        result: Star,
    },
    ArcanaCombo {
        source: (Hermit, Strength),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Hermit, Hanged),
        result: Star,
    },
    ArcanaCombo {
        source: (Hermit, Death),
        result: Strength,
    },
    ArcanaCombo {
        source: (Hermit, Temperance),
        result: Strength,
    },
    ArcanaCombo {
        source: (Hermit, Devil),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Hermit, Tower),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Hermit, Star),
        result: Strength,
    },
    ArcanaCombo {
        source: (Hermit, Moon),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Hermit, Sun),
        result: Devil,
    },
    ArcanaCombo {
        source: (Hermit, Judgement),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Hermit, Faith),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Hermit, Councillor),
        result: Faith,
    },
    ArcanaCombo {
        source: (Fortune, Fortune),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Fortune, Strength),
        result: Faith,
    },
    ArcanaCombo {
        source: (Fortune, Hanged),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Fortune, Death),
        result: Star,
    },
    ArcanaCombo {
        source: (Fortune, Temperance),
        result: Empress,
    },
    ArcanaCombo {
        source: (Fortune, Devil),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Fortune, Tower),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Fortune, Star),
        result: Devil,
    },
    ArcanaCombo {
        source: (Fortune, Moon),
        result: Sun,
    },
    ArcanaCombo {
        source: (Fortune, Sun),
        result: Star,
    },
    ArcanaCombo {
        source: (Fortune, Judgement),
        result: Tower,
    },
    ArcanaCombo {
        source: (Fortune, Faith),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Fortune, Councillor),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Strength, Strength),
        result: Strength,
    },
    ArcanaCombo {
        source: (Strength, Hanged),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Strength, Death),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Strength, Temperance),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Strength, Devil),
        result: Death,
    },
    ArcanaCombo {
        source: (Strength, Tower),
        result: Faith,
    },
    ArcanaCombo {
        source: (Strength, Star),
        result: Moon,
    },
    ArcanaCombo {
        source: (Strength, Moon),
        result: Magician,
    },
    ArcanaCombo {
        source: (Strength, Sun),
        result: Moon,
    },
    ArcanaCombo {
        source: (Strength, Faith),
        result: Star,
    },
    ArcanaCombo {
        source: (Strength, Councillor),
        result: Empress,
    },
    ArcanaCombo {
        source: (Hanged, Hanged),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Hanged, Death),
        result: Moon,
    },
    ArcanaCombo {
        source: (Hanged, Temperance),
        result: Death,
    },
    ArcanaCombo {
        source: (Hanged, Devil),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Hanged, Tower),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Hanged, Star),
        result: Justice,
    },
    ArcanaCombo {
        source: (Hanged, Moon),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Hanged, Sun),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Hanged, Judgement),
        result: Star,
    },
    ArcanaCombo {
        source: (Hanged, Faith),
        result: Devil,
    },
    ArcanaCombo {
        source: (Hanged, Councillor),
        result: Star,
    },
    ArcanaCombo {
        source: (Death, Death),
        result: Death,
    },
    ArcanaCombo {
        source: (Death, Temperance),
        result: Hanged,
    },
    ArcanaCombo {
        source: (Death, Devil),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Death, Tower),
        result: Sun,
    },
    ArcanaCombo {
        source: (Death, Star),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Death, Moon),
        result: Hierophant,
    },
    ArcanaCombo {
        source: (Death, Sun),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Death, Faith),
        result: Fool,
    },
    ArcanaCombo {
        source: (Death, Councillor),
        result: Magician,
    },
    ArcanaCombo {
        source: (Temperance, Temperance),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Temperance, Devil),
        result: Fool,
    },
    ArcanaCombo {
        source: (Temperance, Tower),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Temperance, Star),
        result: Sun,
    },
    ArcanaCombo {
        source: (Temperance, Moon),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Temperance, Sun),
        result: Magician,
    },
    ArcanaCombo {
        source: (Temperance, Judgement),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Temperance, Faith),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Temperance, Councillor),
        result: Fool,
    },
    ArcanaCombo {
        source: (Devil, Devil),
        result: Devil,
    },
    ArcanaCombo {
        source: (Devil, Tower),
        result: Magician,
    },
    ArcanaCombo {
        source: (Devil, Star),
        result: Strength,
    },
    ArcanaCombo {
        source: (Devil, Moon),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Devil, Sun),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Devil, Judgement),
        result: Lovers,
    },
    ArcanaCombo {
        source: (Devil, Faith),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Devil, Councillor),
        result: Chariot,
    },
    ArcanaCombo {
        source: (Tower, Tower),
        result: Tower,
    },
    ArcanaCombo {
        source: (Tower, Star),
        result: Councillor,
    },
    ArcanaCombo {
        source: (Tower, Moon),
        result: Hermit,
    },
    ArcanaCombo {
        source: (Tower, Sun),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Tower, Judgement),
        result: Moon,
    },
    ArcanaCombo {
        source: (Tower, Faith),
        result: Death,
    },
    ArcanaCombo {
        source: (Tower, Councillor),
        result: Death,
    },
    ArcanaCombo {
        source: (Star, Star),
        result: Star,
    },
    ArcanaCombo {
        source: (Star, Moon),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Star, Sun),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Star, Judgement),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Star, Faith),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Star, Councillor),
        result: Sun,
    },
    ArcanaCombo {
        source: (Moon, Moon),
        result: Moon,
    },
    ArcanaCombo {
        source: (Moon, Sun),
        result: Empress,
    },
    ArcanaCombo {
        source: (Moon, Judgement),
        result: Fool,
    },
    ArcanaCombo {
        source: (Moon, Faith),
        result: Sun,
    },
    ArcanaCombo {
        source: (Moon, Councillor),
        result: Temperance,
    },
    ArcanaCombo {
        source: (Sun, Sun),
        result: Sun,
    },
    ArcanaCombo {
        source: (Sun, Judgement),
        result: Death,
    },
    ArcanaCombo {
        source: (Sun, Faith),
        result: Emperor,
    },
    ArcanaCombo {
        source: (Sun, Councillor),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Judgement, Judgement),
        result: Judgement,
    },
    ArcanaCombo {
        source: (Judgement, Faith),
        result: Fortune,
    },
    ArcanaCombo {
        source: (Judgement, Councillor),
        result: Devil,
    },
    ArcanaCombo {
        source: (Faith, Faith),
        result: Faith,
    },
    ArcanaCombo {
        source: (Faith, Councillor),
        result: Priestess,
    },
    ArcanaCombo {
        source: (Councillor, Councillor),
        result: Councillor,
    },
    ArcanaCombo {
        source: (World, World),
        result: World,
    },
];
