use std::collections::HashMap;
use std::sync::Mutex;
use crate::player::Player;
use crate::EnergyT;
use godot::prelude::*;

enum Ownership {
    Battle(usize, [EnergyT; 5]),
    Owned(usize),
    None
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Nebula {
    #[export]
    name: GString,
    #[export]
    energy_yield: EnergyT,

    ownership: Ownership,

    base: Base<Node>
}

#[godot_api]
impl INode for Nebula {
    fn init(base: Base<Node>) -> Self {
        Self {
            name: "".into(),
            energy_yield: 0,
            ownership: Ownership::None,

            base,
        }
    }
}

#[godot_api]
impl Nebula {
    #[func]
    fn get_player(&self) -> Gd<Player> {
        self.base().get_parent().unwrap().cast()
    }
    
    #[func]
    fn commit_energy(&mut self, player_id: i64, commitment: EnergyT) {
        let player_id = player_id as usize;
        
        self.ownership = match self.ownership {
            Ownership::Battle(owner, mut refs) => {
                if owner < refs.len() {
                    refs[owner] += commitment;
                }
                Ownership::Battle(owner, refs)
            }
            Ownership::Owned(owner) => {
                if owner != player_id && owner < 5 {
                    let mut out_array = [0; 5];
                    out_array[owner] = commitment;
                    Ownership::Battle(owner, out_array)
                } else {
                    Ownership::Owned(owner)
                }
            },
            Ownership::None => Ownership::None
        };
    }
}
