use std::ops::{Deref, DerefMut};
use godot::meta::GodotType;
use godot::prelude::*;
use crate::EnergyT;
use crate::nebula::Nebula;

const DEFAULT_COLOR : Color = Color::from_rgb(1.0,1.0,1.0);

enum Faction {
    None,
    Placeholder
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Player {
    name: GString,
    faction: Faction,
    owned_nebula: Vec<Gd<Nebula>>,
    total_energy: EnergyT,
    
    color: Color,

    base: Base<Node>
}

#[godot_api]
impl INode for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            total_energy: 0,
            faction: Faction::None,
            owned_nebula: Vec::new(),
            name: "Unknown Player".into(),
            color: DEFAULT_COLOR,

            base
        }
    }
}

#[godot_api]
impl Player {
    fn start_turn(&mut self) {
        self.total_energy += self.owned_nebula.iter().map(|x| x.bind().energy_yield).sum::<EnergyT>();
    }
}
