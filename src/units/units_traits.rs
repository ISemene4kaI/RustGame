
pub trait Damagable {
    fn damage(&mut self, damage: u32) -> bool;
}

pub trait Healable {
    fn heal(&mut self, heal: u32);
}

pub trait Entity: Damagable + Healable {
    //Getters
    fn get_damage(&self) -> u32;
    fn get_hp(&self) -> u32;
    fn get_armor(&self) -> u8;

    fn get_weapon(&self) -> &'static str;
}
