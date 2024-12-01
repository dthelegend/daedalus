use godot::prelude::*;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct NebulaManager {
    #[export]
    player0: Option<Gd<Player>>,
    #[export]
    player1: Option<Gd<Player>>,
    #[export]
    player2: Option<Gd<Player>>,
    #[export]
    player3: Option<Gd<Player>>,
    #[export]
    player4: Option<Gd<Player>>,

    base: Base<Node>
}

#[godot_api]
impl INode for NebulaManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            player0: None,
            player1: None,
            player2: None,
            player3: None,
            player4: None,
            base
        }
    }
}

#[godot_api]
impl NebulaManager {
    fn get_player(&self, index: usize) -> Option<Gd<Player>> {
        match index {
            0 => self.player0.clone(),
            1 => self.player1.clone(),
            2 => self.player2.clone(),
            3 => self.player3.clone(),
            4 => self.player4.clone(),
            _ => None
        }
    }
}
