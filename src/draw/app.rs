use bevy::prelude::*;

use super::render::{setup_render, render_world};

pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_startup_system(setup_render)
        .add_system(render_world)

        .run();
}
