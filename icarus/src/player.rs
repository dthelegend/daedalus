use crate::EnergyT;
use godot::prelude::*;

const DEFAULT_COLOR: Color = Color::from_rgb(1.0, 1.0, 1.0);

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
enum Faction {
    None,
    Placeholder,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Player {
    #[export]
    name: GString,
    #[export]
    faction: Faction,
    #[export]
    color: Color,
    #[export]
    total_energy: EnergyT,
    
    base: Base<Node>,
}

#[godot_api]
impl INode for Player {
    fn init(base: Base<Node>) -> Self {
        Self {
            total_energy: 0,
            faction: Faction::None,
            name: "Unknown Player".into(),
            color: DEFAULT_COLOR,

            base,
        }
    }
}
