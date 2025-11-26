use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    render::{
        RenderPlugin,
        settings::{
            Backends, InstanceFlags, RenderCreation, WgpuFeatures, WgpuLimits, WgpuSettings,
        },
    },
    window::WindowMode,
    winit::WinitSettings,
};
use safir::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::WARN,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        limits: WgpuLimits::downlevel_defaults(),
                        instance_flags: InstanceFlags::default(),
                        features: WgpuFeatures::TEXTURE_FORMAT_16BIT_NORM,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(WinitSettings::mobile())
        .add_plugins(GamePlugin)
        .run();
}
