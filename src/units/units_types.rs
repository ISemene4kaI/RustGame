
use super::units_traits::{Damagable, Healable, Entity};

//Units Types
pub enum EntityTypes {
    Warriors(Warrior),
    Archers(Archer),
    Mages(Mage)
}


//MeleeWeapon

#[derive(Copy, Clone)]
pub enum MeleeWeapons {
    Stick = 2,
    BaseballBat = 10,
    Sword = 20,
    Axe = 50
}

impl MeleeWeapons {
    pub fn get_damage(&self) -> isize {
        *self as isize
    }

    pub fn name(&self) -> &'static str {
        match self {
            MeleeWeapons::Stick => "Stick",
            MeleeWeapons::BaseballBat => "Baseball Bat",
            MeleeWeapons::Sword => "Sword",
            MeleeWeapons::Axe => "Axe",
        }
    }
}

//RangedWeapons
#[derive(Copy, Clone)]
pub enum RangedWeapons {
    Rock = 5,
    Bow = 15,
    Crossbow = 35,
    Cannon = 70
}

impl RangedWeapons {
    pub fn get_damage(&self) -> isize {
        *self as isize
    }

    pub fn name(&self) -> &'static str {
        match self {
            RangedWeapons::Rock => "Rock",
            RangedWeapons::Bow => "Bow",
            RangedWeapons::Crossbow => "Crossbow",
            RangedWeapons::Cannon => "Cannon",
        }
    }
}

//MageWeapons
#[derive(Copy, Clone)]
pub enum MageWeapons {
    FireStick = 2,
    FireStaff = 22,
    IceStaff = 52,
    PoisonStaff = 82
}
impl MageWeapons {
    pub fn get_damage(&self) -> isize {
        *self as isize
    }

    pub fn name(&self) -> &'static str {
        match self {
            MageWeapons::FireStick => "FireStick",
            MageWeapons::FireStaff => "FireStaff",
            MageWeapons::IceStaff => "IceStaff",
            MageWeapons::PoisonStaff => "PoisonStaff",
        }
    }
}

//Warrior ------------------------------------
pub struct Warrior {
    maxhealth: u32,
    health: u32,
    weapon: MeleeWeapons,
    armor: u8
}

impl Warrior {
    pub fn new(maxhealth: u32, health: u32, weapon: MeleeWeapons, armor: u8) -> Option<Warrior>{
        if health != 0 {
            Some(Warrior {maxhealth, health, weapon, armor})
        }
        else { None }

    }
}

//Archer --------------------------------------------

pub struct Archer {
    maxhealth: u32,
    health: u32,
    weapon: RangedWeapons,
    armor: u8
}

impl Archer {
    pub fn new(maxhealth: u32, health: u32, weapon: RangedWeapons, armor: u8) -> Option<Archer>{
        if health != 0 {
            Some(Archer {maxhealth, health, weapon, armor})
        }
        else { None }

    }
}

//Mage ---------------------------------------------------------------

pub struct Mage {
    maxhealth: u32,
    health: u32,
    weapon: MageWeapons,
    armor: u8
}

impl Mage {
    pub fn new(maxhealth: u32, health: u32, weapon: MageWeapons, armor: u8) -> Option<Mage>{
        if health != 0 {
            Some(Mage {maxhealth, health, weapon, armor})
        }
        else { None }

    }
}

//Traits for Entity Types --------------------------------------------

impl Damagable for EntityTypes {
    fn damage(&mut self, damage: u32) -> bool {
        match self {
            EntityTypes::Warriors(w) => {
                if w.health.checked_sub(damage).is_none() {
                    w.health = 0;
                    true
                } else {
                    w.health -= damage;
                    false
                }
            },
            EntityTypes::Archers(w) => {
                if w.health.checked_sub(damage).is_none() {
                    w.health = 0;
                    true
                } else {
                    w.health -= damage;
                    false
                }
            },
            EntityTypes::Mages(w) => {
                if w.health.checked_sub(damage).is_none() {
                    w.health = 0;
                    true
                } else {
                    w.health -= damage;
                    false
                }
            }
        }
    }
}

impl Healable for EntityTypes {
    fn heal(&mut self, heal: u32) {
        match self {
            EntityTypes::Warriors(w) => {
                if w.health+heal<=w.maxhealth {w.health+=heal}
                else {w.health=w.maxhealth}
            },
            EntityTypes::Archers(w) => {
                if w.health+heal<=w.maxhealth {w.health+=heal}
                else {w.health=w.maxhealth}
            },
            EntityTypes::Mages(w) => {
                if w.health+heal<=w.maxhealth {w.health+=heal}
                else {w.health=w.maxhealth}
            }
        }
    }
}

impl Entity for EntityTypes {
    fn get_damage(&self) -> u32 {
        match self {
            EntityTypes::Warriors(w) => w.weapon.get_damage() as u32,
            EntityTypes::Archers(w) => w.weapon.get_damage() as u32,
            EntityTypes::Mages(w) => w.weapon.get_damage() as u32
        }
    }

    fn get_hp(&self) -> u32 {
        match self {
            EntityTypes::Warriors(w) => w.health,
            EntityTypes::Archers(w) => w.health,
            EntityTypes::Mages(w) => w.health
        }
    }

    fn get_armor(&self) -> u8 {
        match self {
            EntityTypes::Warriors(w) => w.armor,
            EntityTypes::Archers(w) => w.armor,
            EntityTypes::Mages(w) => w.armor
        }
    }
    fn get_weapon(&self) -> &'static str {
        match self {
            EntityTypes::Warriors(w) => w.weapon.name(),
            EntityTypes::Archers(w) => w.weapon.name(),
            EntityTypes::Mages(w) => w.weapon.name()
        }
    }
}
