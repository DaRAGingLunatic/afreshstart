use crate::game::star::resources::*;
use crate::game::star::systems::*;
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<StarSpawnTimer>()
            // Startup systems
            //.add_startup_system(spawn_stars)
            // On Enter State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // Star systems
            // .add_system(tick_star_spawn_timer)
            // .add_system(spawn_stars_over_time);.
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
