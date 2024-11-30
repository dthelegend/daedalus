use godot::classes::{AnimatedSprite2D, IAnimatedSprite2D};
use godot::prelude::*;
use crate::EnergyT;
use crate::player::Player;


#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Nebula {
    #[var]
    owner: Option<Gd<Player>>,
    #[var]
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
    fn set_player(&mut self, player: Gd<Player>) {
        self.owner = Some(player);
    }
}
