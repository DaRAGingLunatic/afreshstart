use bevy::prelude::*;
use bevy::prelude::shape::Quad;

#[derive(Component)]
pub struct PowerUp {}

#[derive(Component)]
pub struct Rocket {
    pub direction: Vec3,
    pub rotation: Quat,
}