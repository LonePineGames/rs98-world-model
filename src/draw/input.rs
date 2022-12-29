use bevy::{prelude::{Input, MouseButton, Res, Query, With, Camera, Vec2, GlobalTransform, ResMut}, render::camera::RenderTarget, window::Windows};

use crate::model::{world::World, auto::AutoNdx, act::Action};

use super::camera::CameraTarget;

pub fn handle_input(
  buttons: Res<Input<MouseButton>>,
  mut q_camera: Query<(&Camera, &GlobalTransform), With<CameraTarget>>,
  windows: Res<Windows>,
  mut world: ResMut<World>,
) {
  // if mouse_input.pressed(MouseButton::Middle) {
  //   let delta = mouse_motion.iter().map(|event| event.delta).sum::<Vec2>();
  //   for mut camera_transform in query_camera.iter_mut() {
  //     camera_transform = 
  //   }
  // }

  // get the camera info and transform
  // assuming there is exactly one main camera entity, so query::single() is OK
  let (camera, camera_matrixes) = q_camera.single_mut();
  
  // get the window that the camera is displaying to (or the primary window)
  let wnd = if let RenderTarget::Window(id) = camera.target {
    windows.get(id).unwrap()
  } else {
    windows.get_primary().unwrap()
  };

  // check if the cursor is inside the window and get its position
  if let Some(screen_pos) = wnd.cursor_position() {
    // get the size of the window
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    println!("screen_pos: {:?}", screen_pos);
    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_matrixes.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_loc = ndc_to_world.project_point3(ndc.extend(-1.0));

    println!("world_loc: {:?}", world_loc);

    // convert to grid coordinates
    let world_pos = world.vec_to_ivec(AutoNdx(0), world_loc);

    println!("world_pos: {:?}", world_pos);

    if buttons.just_pressed(MouseButton::Left) {
      world.set_auto_action(AutoNdx(1), Action::Goto(world_pos));
    }
  }
}
