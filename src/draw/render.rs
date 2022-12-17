
use bevy::{prelude::*, render::camera::ScalingMode, gltf::Gltf};

pub fn render_world() {
}

/// set up a simple 3D scene
pub fn setup_render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {

  /*
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ..default()
  });

  // cubes
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(1.5, 0.5, 1.5),
    ..default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(1.5, 0.5, -1.5),
    ..default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(-1.5, 0.5, 1.5),
    ..default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(-1.5, 0.5, -1.5),
    ..default()
  });*/

  //let my_gltf = ass.load("model/r1000.glb");
  let my_gltf = ass.load("model/r1000.gltf#Scene0");
  let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
  transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
  commands.spawn(SceneBundle {
      scene: my_gltf,
      transform,
      ..Default::default()
  });

  let car_gltf = ass.load("model/sedanSports.glb#Scene0");
  let mut transform = Transform::from_xyz(2.0, 0.0, 0.0);
  transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
  commands.spawn(SceneBundle {
      scene: car_gltf,
      transform,
      ..Default::default()
  });

  let tire_gltf = ass.load("model/wheelDefault.glb#Scene0");
  let mut transform = Transform::from_xyz(-2.0, 0.0, 0.0);
  transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
  commands.spawn(SceneBundle {
      scene: tire_gltf,
      transform,
      ..Default::default()
  });

  /* 
  for x in 0..30 {
    for y in 0..30 {
      
      let tire_gltf = ass.load("model/lab-tile.glb#Scene0");
      let mut transform = Transform::from_xyz(x as f32 - 15.0, y as f32 - 15.0, 0.0);
      transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
      commands.spawn(SceneBundle {
          scene: tire_gltf,
          transform,
          ..Default::default()
      });

    }
  }*/

}