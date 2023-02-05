
use bevy::{render::camera::{ScalingMode, RenderTarget}, input::mouse::MouseWheel, core_pipeline::bloom::BloomSettings, prelude::{Plugin, App, Component, Vec3, Commands, Res, Camera3dBundle, Camera, OrthographicProjection, Transform, default, DirectionalLightBundle, DirectionalLight, Color, Query, With, EventReader, Without, IntoSystemDescriptor, UiCameraConfig, ClearColor}};

use crate::{model::world::World, program::program::ProgramSpace};

use super::{post::RenderImage, app::SSCamera, entities::{self, TrackedEntity, update_entities}};

pub struct RS98CameraPlugin;

impl Plugin for RS98CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(SSCamera, setup_camera)
            .add_system(update_camera.after(update_entities));
    }
}

#[derive(Component)]
pub struct CameraTarget {
  looking_at: Vec3,
  distance: f32,
}

pub fn setup_camera(
  mut commands: Commands,
  render_image: Option<Res<RenderImage>>,
) {
  /*// initial rotation
  let mut rotation = Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4);
  rotation.mul_assign(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4));*/

  // camera
  let mut camera = Camera {
    hdr: true,
    ..default()
  };
  if let Some(render_image) = render_image {
    camera.target = RenderTarget::Image(render_image.image.clone());
  }
  commands.spawn((
    Camera3dBundle {
      camera,
      projection: OrthographicProjection {
        scale: 3.0,
        scaling_mode: ScalingMode::FixedVertical(2.0),
        ..default()
      }.into(),
      transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Z),
      ..default()
    },
    CameraTarget {
      looking_at: Vec3::ZERO,
      distance: 5.0,
    },
    UiCameraConfig { show_ui: true },
    BloomSettings::default()
  ));

  let size = 100.0;
  let shadow_projection = OrthographicProjection {
    left: -size,
    right: size,
    bottom: -size,
    top: size,
    near: -size,
    far: size,
    ..Default::default()
  };

  // light
  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_xyz(5.0, -5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
    directional_light: DirectionalLight {
      color: Color::rgb(1.0, 1.0, 1.0),
      illuminance: 50000.0,
      shadows_enabled: true,
      shadow_projection,
      ..default()
    },
    ..default()
  });

  // diffuse light
  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_xyz(-5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
    directional_light: DirectionalLight {
      color: Color::rgb(0.9, 1.0, 1.0),
      illuminance: 5000.0,
      shadows_enabled: false,
      ..default()
    },
    ..default()
  });

  commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.01)));

}

fn update_camera(
  mut q_camera: Query<(&mut Transform, &mut CameraTarget), With<Camera>>,
  mut q_entity: Query<&mut Transform, (Without<Camera>, Without<DirectionalLight>)>,
  mut q_light: Query<(&mut Transform, &mut DirectionalLight), Without<Camera>>,
  // mouse_input: Res<Input<MouseButton>>,
  // mut mouse_motion: EventReader<MouseMotion>,
  mut scroll_evr: EventReader<MouseWheel>,
  world: Res<World>,
  program: Res<ProgramSpace>,
  entities: Res<entities::Entities>,
) {

  let scroll_delta: f32 = scroll_evr.iter().map(|e| e.y).sum();
  let scroll_delta = scroll_delta * 0.1;

  let mut camera_target = Vec3::ZERO;
  let mut cam_distance = 10.0;

  for (mut transform, mut target) in q_camera.iter_mut() {
    target.distance -= scroll_delta * target.distance;
    target.distance = clamp(target.distance, 0.1, 100.0);
    cam_distance = target.distance;

    let looking_at = world.get_auto(program.access).loc;
    target.looking_at = looking_at.as_vec2().extend(2.0);
    camera_target = target.looking_at;
    let entity = entities.get(TrackedEntity::Auto(program.access));
    if let Some(entity) = entity {
      if let Ok(transform) = q_entity.get_mut(entity) {
        target.looking_at = transform.translation;
      }
    }

    let camera_offset = Vec3::new(1.0, -10.0, 10.0) * target.distance;
    let camera_loc = target.looking_at + camera_offset;
    *transform = Transform::from_translation(camera_loc).looking_at(target.looking_at, Vec3::Z)
        .with_scale(Vec3::splat(target.distance));
  }

  for (mut transform, mut directional_light) in q_light.iter_mut() {
    let size = cam_distance * 20.0;
    let offset = Vec3::new(5.0, -5.0, 10.0);
    let offset = offset.normalize() * (size * 0.5);
    transform.translation = camera_target + offset;
    //println!("size: {}", size);
    directional_light.shadow_projection = OrthographicProjection {
      left: -size,
      right: size,
      bottom: -size,
      top: size,
      near: -size,
      far: size,
      ..default()
    };
  }
}

fn clamp(distance: f32, arg_1: f32, arg_2: f32) -> f32 {
  if distance < arg_1 {
    arg_1
  } else if distance > arg_2 {
    arg_2
  } else {
    distance
  }
}