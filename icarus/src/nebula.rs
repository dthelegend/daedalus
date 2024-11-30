use godot::classes::{AnimatedSprite2D, IAnimatedSprite2D};
use godot::prelude::*;
use crate::EnergyT;
use crate::player::Player;


#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Nebula {
    #[export]
    #[var(get, set = set_owner_custom)]
    owner: Option<Gd<Node>>,
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
    fn set_owner_custom(&mut self, new_owner_opt: Option<Gd<Node>>) {
        let self_gd = self.to_gd();
        
        if let Some(old_owner) = self.owner.clone() {
            if let Ok(mut old_player_owner) = old_owner.try_cast::<Player>() {
                old_player_owner.bind_mut().remove_nebula(&self_gd);
            }
        }
        
        if let Some(new_owner) = new_owner_opt {
            self.owner = Some(new_owner.clone());
            if let Ok(mut new_player_owner) = new_owner.try_cast::<Player>() {
                new_player_owner.bind_mut().add_nebula(self_gd.clone());
            }
        }
    }
}
