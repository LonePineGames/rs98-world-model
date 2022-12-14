use std::ops::MulAssign;

use bevy::{prelude::*, render::camera::{ScalingMode, RenderTarget}, input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, core_pipeline::bloom::BloomSettings};

use super::{post::RenderImage, app::SSCamera};

pub struct RS98CameraPlugin;

impl Plugin for RS98CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(SSCamera, setup_camera)
            .add_system(update_camera);
    }
}

#[derive(Component)]
struct CameraTarget {
  looking_at: Vec3,
  distance: f32,
}

pub fn setup_camera(
  mut commands: Commands,
  render_image: Res<RenderImage>,
) {
  /*// initial rotation
  let mut rotation = Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4);
  rotation.mul_assign(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4));*/

  // camera
  commands.spawn((
    Camera3dBundle {
      camera: Camera {
        hdr: true,
        target: RenderTarget::Image(render_image.image.clone()),
        ..default()
      },
      projection: OrthographicProjection {
        scale: 3.0,
        scaling_mode: ScalingMode::FixedVertical(2.0),
        ..default()
      }.into(),
      transform: Transform::from_xyz(-1.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Z),
      ..default()
    },
    CameraTarget {
      looking_at: Vec3::ZERO,
      distance: 5.0,
    },
    BloomSettings::default()
  ));

  let size = 10.0;
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
    transform: Transform::from_xyz(-5.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Z),
    directional_light: DirectionalLight {
      color: Color::rgb(0.9, 1.0, 1.0),
      illuminance: 10000.0,
      shadows_enabled: true,
      shadow_projection,
      ..default()
    },
    ..default()
  });
}

fn update_camera(
    mut q_camera: Query<(&mut Transform, &mut CameraTarget), With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    // mouse_input: Res<Input<MouseButton>>,
    // mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
  let time_delta = time.delta_seconds();
  let move_delta = time_delta * -10.0;

  let scroll_delta: f32 = scroll_evr.iter().map(|e| e.y).sum();
  let scroll_delta = scroll_delta * 0.1;

  /*
  let rot_delta = if mouse_input.pressed(MouseButton::Middle) {
    mouse_motion.iter().map(|event| event.delta).sum::<Vec2>() * time_delta * -0.1
  } else {
    Vec2::ZERO
  };
  let rot_delta = Vec2::new(-rot_delta.x, rot_delta.y*0.2);*/

  for (mut transform, mut target) in q_camera.iter_mut() {
    if keyboard_input.pressed(KeyCode::Comma) {
      target.looking_at.y += move_delta;
    }
    if keyboard_input.pressed(KeyCode::O) {
      target.looking_at.y -= move_delta;
    }
    if keyboard_input.pressed(KeyCode::A) {
      target.looking_at.x -= move_delta;
    }
    if keyboard_input.pressed(KeyCode::E) {
      target.looking_at.x += move_delta;
    }

    // target.rotation *= Quat::from_rotation_y(rot_delta.x);
    // target.rotation *= Quat::from_rotation_x(rot_delta.y);
    target.distance -= scroll_delta;
    target.distance = clamp(target.distance, 0.1, 100.0);

    let camera_offset = Vec3::new(-1.0, 10.0, 10.0) * target.distance;
    let camera_loc = target.looking_at + camera_offset;
    *transform = Transform::from_translation(camera_loc).looking_at(target.looking_at, Vec3::Z)
        .with_scale(Vec3::splat(target.distance));
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