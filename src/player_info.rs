
use bevy::{prelude::*};

/// READ ONLY, WRITING TO THIS WILL NOT CHANGE THE ACTUAL DATA
#[derive(Resource)]
pub struct PlayerInfo {
    pub position: Vec3,
    pub rotation: Quat,
    pub forward: Vec3,
}