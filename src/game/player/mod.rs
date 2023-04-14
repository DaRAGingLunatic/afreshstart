
use crate::game::player::systems::*;
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;


pub mod components;
pub mod systems;

// in order to get SystemSet derived, we also need Debug, Hash, PartialEq, Eq, Clone

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum PlayerSystemSet {
//     Movement,
//     Confinement
// }
// we can use systemsets as enums as well.
//.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // Startup systems
            // .add_startup_system(spawn_player)
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))

            // Player systems
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                    player_hit_star,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}

// you can explicitly order your systems, by using the .before() method, the .after() method,
// or you could do chaining. .add_systems((player_movement, confine_player_movement).chain()).
// .chain() runs the systems in sequence.
// however explicitly ordering systems too often, weakens performance, as you are not allowing
// opportunities for the system to achieve high multi-threading.
