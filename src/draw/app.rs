use bevy::prelude::*;

use super::{render::{setup_render, render_world}, input::handle_input, camera::RS98CameraPlugin};

pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RS98CameraPlugin)

        .add_startup_system(setup_render)
        .add_system(render_world)
        .add_system(handle_input)

        .run();
}
