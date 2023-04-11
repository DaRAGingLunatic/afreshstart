use bevy::prelude::*;

pub(crate) mod components;
pub(crate) mod resources;
pub(crate) mod systems;

use crate::game::SimulationState;
use crate::AppState;
use resources::*;
use systems::*;

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<PowerUpSpawnTimer>()

            // Startup systems
            // .add_startup_system(spawn_enemies)
            // Enter State systems
            // Spawn enemies will only spawn once on entering AppState::Game

            // Enemy systems
            .add_systems(
                (
                    player_hit_power_up,
                    tick_power_up_spawn_timer,
                    spawn_power_up_over_time,
                    rocket_hit_enemy,
                    rocket_movement,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State systems
            .add_system(despawn_power_up.in_schedule(OnExit(AppState::Game)))
            .add_system(despawn_rocket.in_schedule(OnExit(AppState::Game)));
    }
}
