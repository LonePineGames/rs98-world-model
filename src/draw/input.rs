use bevy::{prelude::{Input, MouseButton, EventReader, Res, Query, Transform, With, Camera, Vec2}, input::mouse::MouseMotion};

pub fn handle_input(
  mouse_input: Res<Input<MouseButton>>,
  mut mouse_motion: EventReader<MouseMotion>,
  mut query_camera: Query<&mut Transform, With<Camera>>,
) {
  // if mouse_input.pressed(MouseButton::Middle) {
  //   let delta = mouse_motion.iter().map(|event| event.delta).sum::<Vec2>();
  //   for mut camera_transform in query_camera.iter_mut() {
  //     camera_transform = 
  //   }
  // }
}
