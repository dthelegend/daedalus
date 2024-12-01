mod nebula;
mod player;
mod nebulamanager;
mod consts;

use godot::prelude::*;

pub type EnergyT = i64;

struct IcarusExtension;

#[gdextension]
unsafe impl ExtensionLibrary for IcarusExtension {}
