use bevy::{prelude::*, window::{CompositeAlphaMode, PresentMode, CursorGrabMode}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::model::world::RS98WorldPlugin;
use crate::program::program::RS98ProgramPlugin;

use super::{input::RS98InputPlugin, camera::RS98CameraPlugin, text::RS98TextPlugin, post::RS98PostPlugin, entities::{RS98EntitiesPlugin}};

#[derive(StageLabel)]
pub struct SSCamera;

#[derive(StageLabel)]
pub struct SSPost;

pub fn start_app() {
    App::new()
        .add_startup_stage(SSPost, SystemStage::parallel())
        .add_startup_stage_after(SSPost, SSCamera, SystemStage::parallel())
        // .add_plugin(AssetPlugin {
        //     watch_for_changes: true,
        //     ..default()
        // })
        .add_plugins(DefaultPlugins.set(WindowPlugin {

            window: WindowDescriptor {
                title: "Robopocalypse Simulator 98".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                //present_mode: PresentMode::AutoVsync, // vsync
                cursor_visible: true,
                //cursor_grab_mode: CursorGrabMode::Confined,
                ..default()
            },
            ..default()
        }).set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        //.add_plugin(WorldInspectorPlugin)
        .add_plugin(RS98WorldPlugin)
        .add_plugin(RS98ProgramPlugin)
        .add_plugin(RS98InputPlugin)
        //.add_plugin(RS98PostPlugin) // comment out this line to disable post-processing
        .add_plugin(RS98CameraPlugin)
        .add_plugin(RS98TextPlugin)
        .add_plugin(RS98EntitiesPlugin)


        .run();
}
