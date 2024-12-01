use std::collections::HashMap;
use std::sync::Mutex;
use crate::player::Player;
use crate::EnergyT;
use godot::prelude::*;
use crate::nebulamanager::NebulaManager;

enum Ownership {
    Battle(usize, [EnergyT; 5]),
    Owned(usize),
    None
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Nebula {
    #[export]
    title: GString,
    #[export]
    energy_yield: EnergyT,
    #[export(range = (1.0,4.0))]
    #[var(get = get_ownership_custom, set = set_ownership_custom)]
    ownership: i64,
    #[export]
    is_homeworld: bool,
    
    _ownership: Ownership,

    base: Base<Node>
}

#[godot_api]
impl INode for Nebula {
    fn init(base: Base<Node>) -> Self {
        Self {
            title: "".into(),
            energy_yield: 0,
            ownership: -1,
            _ownership: Ownership::None,
            is_homeworld: false,

            base,
        }
    }
}

#[godot_api]
impl Nebula {
    #[func]
    fn get_player(&self, player_id: i64) -> Option<Gd<Player>> {
        let player_id = player_id as usize;
        let parent_man = self.base().get_parent().unwrap().cast::<NebulaManager>();
        
        match player_id {
            0 => parent_man.bind().get_player0(),
            1 => parent_man.bind().get_player1(),
            2 => parent_man.bind().get_player2(),
            3 => parent_man.bind().get_player3(),
            4 => parent_man.bind().get_player4(),
            _ => None
        }
    }
    
    #[func]
    fn set_ownership_custom(&mut self, ownership: i64) {
        match ownership {
            a if a >= 0 && a < 5 => self._ownership = Ownership::Owned(a as usize),
            _ => self._ownership = Ownership::None
        };
    }
    
    #[func]
    fn get_ownership_custom(&self) -> i64 {
        match self._ownership {
            Ownership::Battle(i, _) => i as i64,
            Ownership::Owned(i) => i as i64,
            Ownership::None => -1
        }
    }
    
    #[func]
    fn commit_energy(&mut self, player_id: i64, commitment: EnergyT) {
        if self.is_homeworld {
            return;
        }
        
        let player_id = player_id as usize;
        if player_id >= 5 || self.is_homeworld {
            return;
        }
        
        self._ownership = match self._ownership {
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
