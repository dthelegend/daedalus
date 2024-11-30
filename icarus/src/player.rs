use std::ops::{Deref, DerefMut};
use godot::meta::GodotType;
use godot::prelude::*;
use crate::EnergyT;
use crate::nebula::Nebula;

const DEFAULT_COLOR : Color = Color::from_rgb(1.0,1.0,1.0);

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
enum Faction {
    None,
    Placeholder
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Player {
    #[var]
    name: GString,
    #[var]
    faction: Faction,
    #[var]
    color: Color,
    owned_nebulae: Vec<Gd<Nebula>>,
    total_energy: EnergyT,

    base: Base<Node>
}

#[godot_api]
impl INode for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            total_energy: 0,
            faction: Faction::None,
            owned_nebulae: Vec::new(),
            name: "Unknown Player".into(),
            color: DEFAULT_COLOR,

            base
        }
    }
}

#[godot_api]
impl Player {
    fn update_energy(&mut self) {
        self.total_energy += self.owned_nebulae.iter().map(|x| x.bind().energy_yield).sum::<EnergyT>();
    }
    
    pub fn add_nebula(&mut self, new_neb: &Nebula) {
        self.owned_nebulae.push(new_neb.clone());
    }

    pub fn remove_nebula(&mut self, old_neb: &Nebula) {
        self.owned_nebulae.remove()
    }
}
