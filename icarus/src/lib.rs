mod nebula;
mod player;

use godot::prelude::*;

pub type EnergyT = i64;

struct IcarusExtension;

#[gdextension]
unsafe impl ExtensionLibrary for IcarusExtension {}
