use bevy::prelude::*;
use rand::Rng;

pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const ENEMY_BULLET_SPAWN_TIMER: f32 = 4.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Component)]
pub struct EnemyBulletSpawnTimer {
    pub timer: Timer,
}

impl Default for EnemyBulletSpawnTimer {
    fn default() -> EnemyBulletSpawnTimer {
        let mut rng = rand::thread_rng();
        let random_duration = rng.gen_range(1.0..3.0); // Adjust the range for desired randomness
        EnemyBulletSpawnTimer {
            timer: Timer::from_seconds(random_duration, TimerMode::Repeating),
        }
    }
}
