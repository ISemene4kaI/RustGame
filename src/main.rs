mod units;

use crate::units::units_types::{EntityTypes, Warrior, MeleeWeapons};
use crate::units::units_traits::{Entity};

fn main() {
    let newUnit: EntityTypes = EntityTypes::Warriors(
        Warrior::new(100, 100, MeleeWeapons::Stick, 10).unwrap()
    );

    println!("{}", newUnit.get_weapon());
}