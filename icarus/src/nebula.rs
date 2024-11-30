use std::mem::swap;
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
    energy_yield: EnergyT,
    
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
    fn set_owner(&mut self, mut new_owner_opt: Option<Gd<Player>>) {
        let self_gd = self.to_gd();
        if let Some(ref mut new_owner) = new_owner_opt {
            new_owner.bind_mut().add_nebula(self_gd.clone());
        }
        
        swap(&mut self.owner, &mut new_owner_opt);

        if let Some(ref mut old_owner) = new_owner_opt {
            old_owner.bind_mut().remove_nebula(&self_gd);
        }
    }
}
