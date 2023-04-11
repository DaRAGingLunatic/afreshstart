use bevy::a11y::accesskit::Invalid;
use bevy::a11y::accesskit::Invalid::True;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::powerups::components::{PowerUp, Rocket};
use rand::prelude::*;
use crate::game::enemy::components::{Enemy, EnemyBullet};
use crate::game::enemy::systems::{ENEMY_BULLET_SPEED, HALF_ENEMY_BULLET_SIZE, HALF_ENEMY_SIZE};
use crate::game::player::components::Player;
use crate::game::powerups::resources::{PowerUpSpawnTimer};
use crate::game::score::resources::Score;

pub const HALF_PLAYER_SIZE: f32 = 32.0; // Player pixel size
pub const HALF_POWER_UP_SIZE: f32 = 94.0;
pub const ROCKET_SPEED: f32 = 400.0;
pub const HALF_ROCKET_SIZE: f32 = 18.0;


pub fn despawn_power_up(mut commands: Commands, power_up_query: Query<Entity, With<PowerUp>>) {
    for power_up_entity in power_up_query.iter() {
        commands.entity(power_up_entity).despawn();
    }
}

pub fn despawn_rocket(mut commands: Commands, rocket_query: Query<Entity, With<Rocket>>) {
    for rocket_entity in rocket_query.iter() {
        commands.entity(rocket_entity).despawn();
    }
}

pub fn tick_power_up_spawn_timer(mut power_up_spawn_timer: ResMut<PowerUpSpawnTimer>, time: Res<Time>) {
    power_up_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_power_up_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    power_up_spawn_timer: Res<PowerUpSpawnTimer>,
) {
    if power_up_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/laserYellow_burst.png"),
                ..default()
            },
            PowerUp {},
        ));
    }
}


fn angle_between_vectors(a: Vec3, b: Vec3) -> f32 {
    let dot = a.dot(b);
    let cross = a.cross(b);
    let angle = dot.acos();
    if cross.z >= 0.0 {
        angle
    } else {
        -angle
    }
}

pub fn player_hit_power_up(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut power_up_query: Query<(Entity, &Transform), With<PowerUp>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
    time: Res<Time>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for (power_up_entity, power_up_transform) in power_up_query.iter_mut() {
            let player_position = player_transform.translation;
            let distance = player_transform
                .translation
                .distance(power_up_transform.translation);
            if distance < HALF_PLAYER_SIZE + HALF_POWER_UP_SIZE {
                println!("Player hit a Power Up!!!! 10 Points!!!!");
                score.value += 10;
                let sound_effect = asset_server.load("audio/forceField_003.ogg");
                audio.play(sound_effect);
                commands.entity(power_up_entity).despawn();
                for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
                    let enemy_position = enemy_transform.translation;
                    let forward_axis = Vec3::new(0.0, 1.0, 0.0); // Assuming the rocket's forward axis is aligned with the positive Y-axis
                    let direction = (enemy_position - player_position).normalize();
                    let rotation_angle = angle_between_vectors(forward_axis, direction);
                    let rotation = Quat::from_rotation_z(rotation_angle);

                    commands.spawn(
                        SpriteBundle {
                            transform: Transform {
                                translation: player_position,
                                rotation,
                                ..Default::default()
                            },
                            texture: asset_server.load("sprites/spaceMissiles_040.png"),
                            ..Default::default()
                        }
                    )
                        .insert(Rocket { direction, rotation });

                }
            }
        }
    }
}

/*
pub fn enemy_bullet_movement(
    mut enemy_bullet_query: Query<(&mut Transform, &EnemyBullet)>,
    time: Res<Time>,
) {
    for (mut transform, enemy_bullet) in enemy_bullet_query.iter_mut() {
        let direction = Vec3::new(enemy_bullet.direction.x, enemy_bullet.direction.y, 0.0);
        transform.translation += direction * ENEMY_BULLET_SPEED * time.delta_seconds();
    }
}

*/



pub fn rocket_movement(
    mut rocket_query: Query<(&mut Transform, &Rocket)>,
    time: Res<Time>,
) {
    for (mut rocket_transform, rocket) in rocket_query.iter_mut() {
        let direction = Vec3::new(rocket.direction.x, rocket.direction.y, 0.0);
        rocket_transform.translation += direction * ROCKET_SPEED * time.delta_seconds();
    }
}

pub fn rocket_hit_enemy(
    mut commands: Commands,
    mut rocket_query: Query<(Entity, &Transform), With<Rocket>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut enemy_bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    for (rocket_entity, rocket_transform) in rocket_query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            let distance = rocket_transform.translation.distance(enemy_transform.translation);
            if distance < HALF_ENEMY_SIZE + HALF_ROCKET_SIZE {
                println!("Rocket hit enemy!!! 5 points!!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(enemy_entity).despawn();
                commands.entity(rocket_entity).despawn();
                score.value += 5;
            }
        }
    }
    for (rocket_entity, rocket_transform) in rocket_query.iter_mut() {
        for (enemy_bullet_entity, enemy_bullet_transform) in enemy_bullet_query.iter_mut() {
            let distance = rocket_transform.translation.distance(enemy_bullet_transform.translation);
            if distance < HALF_ENEMY_BULLET_SIZE + HALF_ROCKET_SIZE {
                println!("Rocket hit enemy bullet!!! 2 points!!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(enemy_bullet_entity).despawn();
                commands.entity(rocket_entity).despawn();
                score.value += 2;
            }
        }
    }
}

