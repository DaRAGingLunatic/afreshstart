use bevy::prelude::*;

pub(crate) mod components;
pub(crate) mod resources;
pub(crate) mod systems;

use crate::game::SimulationState;
use crate::AppState;
use resources::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<EnemyBulletSpawnTimer>()
            // Startup systems
            // .add_startup_system(spawn_enemies)
            // Enter State systems
            // Spawn enemies will only spawn once on entering AppState::Game
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Enemy systems
            .add_systems(
                (
                    enemy_movement,
                    enemy_bullet_movement,
                    enemy_hit_player,
                    enemy_bullet_hit_player,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemy_over_time,
                    tick_enemy_bullet_spawn_timer,
                    spawn_enemy_bullet_over_time,
                    despawn_enemies_bullets_on_screen_exit,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
            .add_system(despawn_enemies_bullets.in_schedule(OnExit(AppState::Game)));
    }
}
