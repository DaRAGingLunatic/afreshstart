use bevy::prelude::*;


#[derive(Component)]
pub struct PowerUp {}

#[derive(Component)]
pub struct Rocket {
    pub direction: Vec3,
    pub rotation: Quat,
    pub target_enemy: Entity,
}