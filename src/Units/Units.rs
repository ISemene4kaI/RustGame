use Units::Traits;

mod units {

    pub struct Warrior {
        health: u32,
        damage: u32,
        armor: u8
    }

    impl Warrior {
        pub fn new(health: u32, damage: u32, armor: u8) -> Option<Warrior>{
            if health != 0 {
                Warrior { health, damage, armor }
            }
            else { None }

        }
    }

    impl Damagable for Warrior {
        fn damage(&mut self, damage: u32) -> bool{
            if assert_eq!(self.health.checked_sub(damage), None){
                self.health = 0;
                true;
            } else {
                self.health -= damage;
                false;
            }
        }
    }

}