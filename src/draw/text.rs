use bevy::prelude::*;

pub struct RS98TextPlugin;

impl Plugin for RS98TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_text);
    }
}

fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/IBMPlexMono-Regular.ttf");
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Hello ".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "World!".to_string(),
                    style: TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::RED,
                    },
                },
            ],
            alignment: Default::default(),
        },
        style: Style {
            align_self: AlignSelf::FlexStart,
            ..Default::default()
        },
        ..Default::default()
    });
}