use crate::game::score::resources::*;
use crate::game::score::systems::*;
use crate::AppState;
use bevy::prelude::*;

pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            // .init_resource::<Score>()
            .init_resource::<HighScores>()
            // On Enter State
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Score systems
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
