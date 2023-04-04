use crate::game::camera::systems::*;
use bevy::prelude::*;

pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup systems
            .add_startup_system(spawn_camera);
    }
}
