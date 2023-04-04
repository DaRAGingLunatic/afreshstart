use bevy::prelude::*;

pub mod camera;
pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
pub(crate) mod ui;

use crate::events::GameOver;
use crate::AppState;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use crate::game::ui::GameUIPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // State
            .add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(GameUIPlugin)
            // systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // the toggle simulation above will only run if we are in the AppState::Game state.
            // On Enter System
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default] // The default SimulationState is Running.
    Running,
    Paused,
}
