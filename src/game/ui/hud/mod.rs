mod components;
mod styles;
mod systems;

use systems::layout::*;

//use crate::game::ui::hud::systems::updates::{update_enemy_text, update_score_text};
use crate::AppState;
use bevy::prelude::*;
use crate::game::ui::hud::systems::interactions::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_hud_menu.in_schedule(OnEnter(AppState::Game)))
            // Systems
            // .add_systems((update_score_text, update_enemy_text).in_set(OnUpdate(AppState::Game)))
            .add_systems((
                interact_with_miner_button,
                interact_with_axeman_button,
                interact_with_axe_thrower_button,
                interact_with_shield_and_sword_button,
                interact_with_knight_and_horse_button,
                interact_with_ogre_magi_button,
                interact_with_giant_sloth_button
            ).in_set(OnUpdate(AppState::Game)))
            // OnExit Systems
            .add_system(despawn_hud_menu.in_schedule(OnExit(AppState::Game)));
    }
}
