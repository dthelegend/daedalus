use godot::prelude::*;
use crate::nebula::Nebula;
use crate::player::Player;

#[derive(GodotClass)]
#[class(init, base = Object)]
struct Decision {
    #[var]
    player: Option<Gd<Player>>,
    #[var]
    enemy_player: Option<Gd<Player>>,
    #[var]
    relevant_nebula: Option<Gd<Nebula>>,
    #[var]
    options: Array<Variant>,

    base: Base<Object>
}