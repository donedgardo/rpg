use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}

impl CameraPlugin {
    pub fn setup(mut commands: Commands) {
        commands.spawn(Camera3dBundle::default());
    }
}
