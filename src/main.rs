pub mod events;
pub mod game;
pub mod main_menu;
pub mod systems;


use bevy::a11y::accesskit::Action;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use systems::*;


fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Ball Game with Bullets".into(),
                mode: WindowMode::SizedFullscreen,
                resolution: (3840., 2160.).into(),
                decorations: true,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        // My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
                // systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}
// in order to get States, we need to have all these other traits derived.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default] // our default game state is set to the MainMenu
    MainMenu,
    Game,
    GameOver,
    Paused,
}
