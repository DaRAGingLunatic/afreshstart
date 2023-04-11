use bevy::prelude::*;
use rand::Rng;

pub const POWER_UP_SPAWN_TIME: f32 = 10.0;

#[derive(Resource, Component)]
pub struct PowerUpSpawnTimer {
    pub timer: Timer,
}

impl Default for PowerUpSpawnTimer {
    fn default() -> PowerUpSpawnTimer {
        let mut rng = rand::thread_rng();
        let random_duration = rng.gen_range(7.0..14.0); // Adjust the range for desired randomness
        PowerUpSpawnTimer {
            timer: Timer::from_seconds(random_duration, TimerMode::Repeating),
        }
    }
}

