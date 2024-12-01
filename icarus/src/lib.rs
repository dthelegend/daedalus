mod nebula;
mod player;
mod nebulamanager;
mod consts;
mod decision;

use godot::prelude::*;

pub type EnergyT = i64;
pub type PopulationT = i64;
pub type HappinessT = i64;

struct IcarusExtension;

#[gdextension]
unsafe impl ExtensionLibrary for IcarusExtension {}
