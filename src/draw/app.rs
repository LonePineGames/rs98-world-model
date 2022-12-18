use bevy::prelude::*;

use crate::model::world::RS98WorldPlugin;

use super::{render::{setup_render, render_world}, input::handle_input, camera::RS98CameraPlugin, text::RS98TextPlugin, post::RS98PostPlugin, entities::{RS98EntitiesPlugin, Entities}};

#[derive(StageLabel)]
pub struct SSCamera;

#[derive(StageLabel)]
pub struct SSPost;

pub fn start_app() {
    App::new()
        .add_startup_stage(SSPost, SystemStage::parallel())
        .add_startup_stage_after(SSPost, SSCamera, SystemStage::parallel())
        .add_plugins(DefaultPlugins)
        .add_plugin(RS98WorldPlugin)
        .add_plugin(RS98PostPlugin)
        .add_plugin(RS98CameraPlugin)
        .add_plugin(RS98TextPlugin)
        .add_plugin(RS98EntitiesPlugin)

        //.add_startup_system(setup_render)
        //.add_system(render_world)
        .add_system(handle_input)

        .run();
}
