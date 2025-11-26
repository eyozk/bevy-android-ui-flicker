use bevy::{
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    prelude::*,
    render::{
        RenderApp, RenderStartup,
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphExt, ViewNodeRunner},
    },
    ui_render::graph::NodeUi,
};

pub use vfx::PostProcessSettings;

mod vfx;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<vfx::PostProcessSettings>::default(),
            UniformComponentPlugin::<vfx::PostProcessSettings>::default(),
        ))
        .add_systems(Update, vfx::update_time);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(RenderStartup, vfx::init_post_process_pipeline);

        render_app
            .add_render_graph_node::<ViewNodeRunner<vfx::PostProcessNode>>(
                Core2d,
                vfx::PostProcessLabel,
            )
            .add_render_graph_edges(
                Core2d,
                (NodeUi::UiPass, vfx::PostProcessLabel, Node2d::Upscaling),
            );
    }
}
