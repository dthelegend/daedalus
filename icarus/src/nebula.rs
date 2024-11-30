use godot::classes::{AnimatedSprite2D, IAnimatedSprite2D};
use godot::prelude::*;
use crate::EnergyT;
use crate::player::Player;


#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Nebula {
    #[export]
    #[var(get, set = set_owner)]
    owner: Option<Gd<Player>>,
    #[export]
    pub energy_yield: EnergyT,
    
    base: Base<AnimatedSprite2D>,
}

#[godot_api]
impl IAnimatedSprite2D for Nebula {
    fn init(base: Base<AnimatedSprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            owner: None,
            energy_yield: 0,
            
            base,
        }
    }
}

#[godot_api]
impl Nebula {
    #[func]
    fn set_owner(&mut self, new_owner: Gd<Player>) {
        new_owner.bind().add_nebula(self);
        self.owner = Some(new_owner);
    }
}
