use bevy::{prelude::*, render::camera::ScalingMode, input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}};

pub struct RS98CameraPlugin;

#[derive(Component)]
struct CameraTarget {
  looking_at: Vec3,
  rotation: Quat,
  distance: f32,
}

impl Plugin for RS98CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(update_camera);
    }
}

pub fn setup_camera(mut commands: Commands) {
  let rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
  // camera
  commands.spawn(Camera3dBundle {
    projection: OrthographicProjection {
      scale: 3.0,
      scaling_mode: ScalingMode::FixedVertical(2.0),
      ..default()
    }.into(),
    transform: Transform::from_xyz(-1.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Z),
    ..default()
  }).insert(CameraTarget {
    looking_at: Vec3::ZERO,
    rotation,
    distance: 5.0,
  });
}

fn update_camera(
    mut query: Query<(&mut Transform, &mut CameraTarget), With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
  let delta = time.delta_seconds();

  let scroll_delta: f32 = scroll_evr.iter().map(|e| e.y).sum();
  let scroll_delta = scroll_delta * 0.1;
  let rot_delta = if mouse_input.pressed(MouseButton::Middle) {
    mouse_motion.iter().map(|event| event.delta).sum::<Vec2>() * delta * -0.1
  } else {
    Vec2::ZERO
  };

  for (mut transform, mut target) in query.iter_mut() {
    if keyboard_input.pressed(KeyCode::Comma) {
      target.looking_at.y += delta;
    }
    if keyboard_input.pressed(KeyCode::O) {
      target.looking_at.y -= delta;
    }
    if keyboard_input.pressed(KeyCode::A) {
      target.looking_at.x -= delta;
    }
    if keyboard_input.pressed(KeyCode::E) {
      target.looking_at.x += delta;
    }

    target.rotation *= Quat::from_rotation_y(rot_delta.x);
    target.rotation *= Quat::from_rotation_x(rot_delta.y);
    target.distance += scroll_delta;
    target.distance = clamp(target.distance, 0.1, 100.0);

    let camera_loc = target.looking_at + target.rotation * Vec3::new(0.0, 0.0, target.distance*10.0);
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