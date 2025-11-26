use bevy::prelude::*;

use crate::post_process::PostProcessSettings;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d, PostProcessSettings { time: 0.0 }));
}
