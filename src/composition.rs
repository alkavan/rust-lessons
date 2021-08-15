use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::ops::{AddAssign, SubAssign};

const MAX_LEVEL: u8 = 5;
const BASE_HEALTH: i32 = 500;
const BASE_XP: u64 = 800;
const BASE_ATTACK: u32 = 2;
const BASE_DEFENCE: u32 = 1;

struct Character {
    name: String,
    level: u8,
}

struct Progress {
    next_level: u64,
    experience: u64,
}

impl Progress {
    fn add_exp(&mut self, amount: u64) {
        self.experience.add_assign(amount);
    }

    fn is_level_up(&self) -> bool {
        return self.experience >= self.next_level;
    }
}

#[derive(Debug)]
enum NpcType {
    Zombie,
    Witch,
    Ghost,
}

impl fmt::Display for NpcType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait CharacterName<T> {
    fn name(&self) -> String;
}

struct Player {
    character: Character,
    health: Health,
    attribute: Attribute,
    progress: Progress,
}

impl CharacterName<Player> for Player {
    fn name(&self) -> String {
        return self.character.name.clone();
    }
}

struct Npc {
    character: Character,
    health: Health,
    attribute: Attribute,
}

impl CharacterName<Npc> for Npc {
    fn name(&self) -> String {
        return self.character.name.clone();
    }
}

#[derive(Clone)]
struct Health {
    max: i32,
    left: i32,
}

impl Health {
    fn for_level(level: u8) -> Health {
        let base = level as i32 * BASE_HEALTH;
        let min = base * 0.75 as i32;
        let max = base * 1.25 as i32;

        let health = rand::thread_rng().gen_range(min..max);

        return Health {
            max: health,
            left: health,
        };
    }

    fn is_dead(&self) -> bool {
        self.left <= 0
    }

    fn is_alive(&self) -> bool {
        return self.left > 0;
    }

    fn fill(&mut self, amount: u16) {
        let missing = self.max - self.left;

        let amount = match amount as i32 > missing {
            true => missing,
            false => amount as i32,
        };

        self.left.add_assign(amount);
    }

    fn drain(&mut self, amount: u16) {
        self.left.sub_assign(amount as i32);
    }
}

#[derive(Clone)]
struct Attribute {
    attack: u8,
    defence: u8,
    experience: u64,
}

impl Attribute {
    fn for_npc_level(level: u8) -> Attribute {
        Attribute {
            attack: 1 * level,
            defence: 1 * level,
            experience: (level as u64 * BASE_XP),
        }
    }

    fn for_player_level(level: u8) -> Attribute {
        Attribute {
            attack: 2 * level,
            defence: 2 * level,
            experience: 0,
        }
    }
}

trait Fighting<Other> {
    fn fight(&mut self, other: &mut Other);
}

impl Fighting<Npc> for Player {
    fn fight(&mut self, other: &mut Npc) {
        const DMG_MUL: f64 = 100.0;

        while self.health.is_alive() && !other.health.is_dead() {
            let player_attack = (self.attribute.attack * self.character.level) as i32;
            let other_defence = (other.attribute.defence * other.character.level) as i32;
            let player_damage = ((player_attack - other_defence) as f64 * DMG_MUL) as i32;

            if player_damage > 0 {
                other.health.drain(player_damage as u16);

                println!(
                    "{} [hit] {}: [a: {}, d: {}, d: {}]",
                    self.name(),
                    other.name(),
                    player_attack,
                    other_defence,
                    player_damage
                );
            } else {
                println!(
                    "{} [block] {} [dmg: {}]",
                    other.name(),
                    self.name(),
                    player_damage
                )
            }

            if other.health.is_alive() {
                let other_attack = (other.attribute.attack * other.character.level) as i32;
                let player_defence = (self.attribute.defence * self.character.level) as i32;
                let other_damage = ((other_attack - player_defence) as f64 * DMG_MUL) as i32;

                if other_damage > 0 {
                    self.health.drain(other_damage as u16);

                    println!(
                        "{} [hit] {}: [atk: {}, def: {}, dmg: {}]",
                        other.name(),
                        self.name(),
                        other_attack,
                        player_defence,
                        other_damage
                    );
                } else {
                    println!(
                        "{} [block] {} [dmg: {}]",
                        other.name(),
                        self.name(),
                        player_damage
                    )
                }
                if self.health.is_dead() {
                    println!("{} is DEAD!", self.name());
                }
            } else {
                println!("{} is DEAD!", other.name());
            }
        }
    }
}

fn new_player(name: &str) -> Player {
    let level: u8 = 1;
    let name = name.to_string();

    let character = Character { name, level };

    let experience_map = get_player_experience_map();
    let next_level = experience_map.get(&level).unwrap().clone();
    let experience = 0;
    let progress = Progress {
        next_level,
        experience,
    };

    let player_health_map = get_player_health_map();
    let health = player_health_map.get(&level).unwrap().clone();

    let attribute = Attribute::for_player_level(level);

    return Player {
        character,
        health,
        attribute,
        progress,
    };
}

fn new_npc(npc_type: NpcType, level: u8) -> Npc {
    let name = npc_type.to_string();

    let character = Character { name, level };

    let health = Health::for_level(level);

    let attribute = Attribute::for_npc_level(level);

    return Npc {
        character,
        health,
        attribute,
    };
}

fn get_player_health_map() -> HashMap<u8, Health> {
    let mut map: HashMap<u8, Health> = HashMap::new();

    for (i, n) in [2, 3, 5, 8, 13].iter().enumerate() {
        map.insert(
            (i + 1) as u8,
            Health {
                max: BASE_HEALTH * n,
                left: BASE_HEALTH * n,
            },
        );
    }
    return map;
}

fn get_player_experience_map() -> HashMap<u8, u64> {
    let mut map: HashMap<u8, u64> = HashMap::new();

    for (i, n) in [2, 3, 5, 8, 13].iter().enumerate() {
        map.insert((i + 1) as u8, (i + 1) as u64 * n);
    }

    return map;
}

pub fn rpg() {
    let mut player = new_player("Quil");

    let mut npc1 = new_npc(NpcType::Witch, 1);

    player.fight(&mut npc1);
}
