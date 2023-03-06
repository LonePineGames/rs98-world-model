use bevy::{prelude::*, render::camera::RenderTarget, window::Windows};
use conniver::p;

use crate::{program::program::ProgramSpace};

use super::camera::CameraTarget;

pub struct RS98InputPlugin;

impl Plugin for RS98InputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(handle_mouse_input)
      .add_system(handle_keyboard_input);
  }
}

pub fn handle_mouse_input(
  buttons: Res<Input<MouseButton>>,
  mut q_camera: Query<(&Camera, &GlobalTransform), With<CameraTarget>>,
  windows: Res<Windows>,
  mut program: ResMut<ProgramSpace>,
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

    //println!("screen_pos: {:?}", screen_pos);
    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_matrixes.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_loc0 = ndc_to_world.project_point3(ndc.extend(0.0));
    let world_loc1 = ndc_to_world.project_point3(ndc.extend(1.0));
    let z_lerp = world_loc0.z / (world_loc0.z - world_loc1.z);
    let world_loc = world_loc0.lerp(world_loc1, z_lerp);
    let world_loc = world_loc + Vec3::new(0.5, 0.5, 0.0);
    let world_pos = world_loc.as_ivec3().truncate();
    //println!("world_loc: {:?}", world_loc);
    // let world_pos = Vec2::new(world_loc.x + 0.5, world_loc.y + world_loc.z + 0.5);
    // let world_pos = IVec2::new(world_pos.x.floor() as i32, world_pos.y.floor() as i32);

    // convert to grid coordinates
    //let world_pos = world_loc.as_ivec3().truncate();
    //let world_pos = world.vec_to_ivec(AutoNdx(0), world_loc);

    //println!("world_pos: {:?}", world_pos);

    if buttons.just_pressed(MouseButton::Left) {
      let event = p(&format!("(input-mouse {} {})", world_pos.x, world_pos.y));
      println!("event: {:?}", event);
      let access = program.access;
      program.interrupt(access, event);
    }
  }
}


pub fn handle_keyboard_input(
  keys: Res<Input<KeyCode>>,
  mut program: ResMut<ProgramSpace>,
) {
  for key in keys.get_pressed() {
    let event = p(&format!("(input-key {:?})", key));
    let access = program.access;
    program.interrupt(access, event);
  }
}