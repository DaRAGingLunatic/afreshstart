use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct EnemyBullet {
    pub direction: Vec3,
}
