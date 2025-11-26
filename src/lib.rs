use bevy::prelude::*;

mod camera;
mod menus;
mod post_process;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            post_process::PostProcessPlugin,
            menus::MenusPlugin,
            camera::CameraPlugin,
        ));
    }
}
