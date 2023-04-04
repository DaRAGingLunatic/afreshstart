use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::*;

use crate::Entity;

use crate::events::GameOver;
use crate::game::enemy::components::{Enemy, EnemyBullet};
use crate::game::enemy::resources::{EnemyBulletSpawnTimer, EnemySpawnTimer};
use crate::game::player::components::Player;
use crate::game::player::systems::HALF_PLAYER_SIZE;
use crate::game::score::resources::Score;

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // Player pixel size
pub const HALF_ENEMY_SIZE: f32 = 32.0; // Player pixel size
pub const NUMBER_OF_ENEMIES: usize = 4; // Player pixel size
pub const ENEMY_BULLET_SIZE: f32 = 32.0;
pub const HALF_ENEMY_BULLET_SIZE: f32 = 16.0;
pub const ENEMY_BULLET_SPEED: f32 = 400.0;
pub const MINIMUM_ENEMY_SPAWN_DISTANCE: f32 = 800.0;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width() - ENEMY_SIZE;
        let random_y = random::<f32>() * window.height() - ENEMY_SIZE;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn despawn_enemies_bullets(
    mut commands: Commands,
    enemy_bullet_query: Query<Entity, With<EnemyBullet>>,
) {
    for enemy_bullet_entity in enemy_bullet_query.iter() {
        commands.entity(enemy_bullet_entity).despawn();
    }
}

pub fn despawn_enemies_bullets_on_screen_exit(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    let window = window_query.single();

    let x_min = 0.0;
    let x_max = window.width();
    let y_min = 0.0;
    let y_max = window.height();

    for (enemy_bullet_entity, enemy_bullet_transform) in enemy_bullet_query.iter_mut() {
        let translation = enemy_bullet_transform.translation;

        if translation.x < x_min
            || translation.x > x_max
            || translation.y < y_min
            || translation.y > y_max
        {
            commands.entity(enemy_bullet_entity).despawn();
        }
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_bullet_movement(
    mut enemy_bullet_query: Query<(&mut Transform, &EnemyBullet)>,
    time: Res<Time>,
) {
    for (mut transform, enemy_bullet) in enemy_bullet_query.iter_mut() {
        let direction = Vec3::new(enemy_bullet.direction.x, enemy_bullet.direction.y, 0.0);
        transform.translation += direction * ENEMY_BULLET_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = HALF_ENEMY_SIZE;
    let x_max = window.width() - (HALF_ENEMY_SIZE + 4.0);
    let y_min = HALF_ENEMY_SIZE;
    let y_max = window.height() - (HALF_ENEMY_SIZE + 33.0);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play SPX
        if direction_changed {
            // Play Sound Effect
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            // Randomly play one of the two sound effects.
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = HALF_ENEMY_SIZE;
    let x_max = window.width() - (HALF_ENEMY_SIZE + 4.0);
    let y_min = HALF_ENEMY_SIZE;
    let y_max = window.height() - (HALF_ENEMY_SIZE + 33.0);

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
// Entities are just u32's. As such we can just copy it around.
pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < HALF_PLAYER_SIZE + HALF_ENEMY_SIZE {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn enemy_bullet_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<EnemyBullet>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_bullet_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_bullet_transform.translation);
            if distance < HALF_PLAYER_SIZE + HALF_ENEMY_BULLET_SIZE {
                println!("Enemy Bullet hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
) {
    if enemy_spawn_timer.timer.finished() {
        if let Ok(player_transform) = player_query.get_single() {
            let player_position = player_transform.translation;
            let window = window_query.get_single().unwrap();
            let mut random_x;
            let mut random_y;
            let mut distance;

            loop {
                random_x = random::<f32>() * window.width();
                random_y = random::<f32>() * window.height();
                distance = player_position.distance(Vec3::new(random_x, random_y, 0.0));

                if distance >= MINIMUM_ENEMY_SPAWN_DISTANCE {
                    break;
                }
            }

            let mut rng = rand::thread_rng();
            let random_duration = rng.gen_range(1.0..3.0); // Adjust the range for desired randomness
            commands
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..Default::default()
                })
                .insert(Enemy {
                    direction: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
                })
                .insert(EnemyBulletSpawnTimer {
                    timer: Timer::from_seconds(random_duration, TimerMode::Repeating),
                });

            enemy_spawn_timer.timer.reset();
        }
    }
}

pub fn tick_enemy_bullet_spawn_timer(
    mut query: Query<&mut EnemyBulletSpawnTimer, With<Enemy>>,
    time: Res<Time>,
) {
    for mut timer in query.iter_mut() {
        timer.timer.tick(time.delta());
    }
}

pub fn spawn_enemy_bullet_over_time(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut EnemyBulletSpawnTimer), With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_position = player_transform.translation;
        for (_, enemy_transform, mut enemy_bullet_spawn_timer) in enemy_query.iter_mut() {
            if enemy_bullet_spawn_timer.timer.finished() {
                let enemy_position = enemy_transform.translation;

                let direction = (player_position - enemy_position).normalize();

                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("sprites/ball_red_small.png").into(),
                        transform: Transform::from_translation(enemy_position),
                        ..default()
                    })
                    .insert(EnemyBullet { direction });

                // Reset the timer with a new random duration
                //let mut rng = rand::thread_rng();
                //let random_duration = rng.gen_range(1.0..3.0); // Adjust the range for desired randomness
                enemy_bullet_spawn_timer.timer.reset();
            }
        }
    }
}
