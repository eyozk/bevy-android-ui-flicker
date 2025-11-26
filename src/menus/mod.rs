use bevy::prelude::*;

mod home;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, home::spawn_home)
            .add_systems(Update, home::update_elapsed_time);
    }
}
