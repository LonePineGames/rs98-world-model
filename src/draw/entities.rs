use std::collections::HashMap;

use bevy::{prelude::*, scene::SceneBundle};

use crate::model::{world::World, auto::AutoNdx, kind::Kind};

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TrackedEntity {
  Auto(AutoNdx),
  Tile(AutoNdx, usize),
  Item(AutoNdx, usize),
}

pub fn update_entity(
  tracker: TrackedEntity,
  loc: Vec3,
  kind: Kind,
  commands: &mut Commands,
  world: &Res<World>,
  entities: &mut ResMut<Entities>,
  ass: &Res<AssetServer>,
  q: &mut Query<&mut Transform, With<TrackedEntity>>,
) {
  let entity = entities.get(tracker);
  if let Some(entity) = entity {
    if let Ok(mut transform) = q.get_mut(entity) {
      transform.translation = loc;
    }
  } else if kind != Kind(0) {
    let data = world.kinds.get_data(kind);
    let scene = ass.load(data.scene.clone());
    let mut transform = Transform::from_translation(loc);
    transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
    let entity = commands.spawn((SceneBundle {
        scene,
        transform,
        ..Default::default()
    }, tracker)).id();
    entities.set(tracker, entity);
  }
}

pub fn update_entities(
  mut commands: Commands,
  world: Res<World>,
  mut entities: ResMut<Entities>,
  ass: Res<AssetServer>,
  mut q: Query<&mut Transform, With<TrackedEntity>>,
) {
  for (auto_ndx, auto) in world.autos.iter().enumerate() {
    let auto_ndx = AutoNdx(auto_ndx);

    let auto_tracker = TrackedEntity::Auto(auto_ndx);
    let auto_entity = entities.get(auto_tracker);
    let auto_loc = auto.loc.as_vec2().extend(0.0);
    update_entity(auto_tracker, auto_loc, auto.kind, &mut commands, &world, &mut entities, &ass, &mut q);
    // if let Some(entity) = auto_entity {
    //   if let Ok(mut transform) = q.get_mut(entity) {
    //     transform.translation = auto_loc;
    //   }
    // } else {
    //   let auto_gltf = ass.load("model/r1000.gltf#Scene0");
    //   let mut transform = Transform::from_translation(auto_loc);
    //   transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
    //   let entity = commands.spawn((SceneBundle {
    //       scene: auto_gltf,
    //       transform,
    //       ..Default::default()
    //   }, auto_tracker)).id();
    //   entities.set(auto_tracker, entity);
    // }

    // tiles
    for (loc, tile) in auto.tiles.iter().enumerate() {
      let tracker = TrackedEntity::Tile(auto_ndx, loc);
      let entity = entities.get(tracker);
      let loc = auto_loc + auto.ndx_to_loc(loc).as_vec2().extend(0.0);
      if let Some(entity) = entity {
        // if let Ok(mut transform) = q.get_mut(entity) {
        //   transform.translation = loc;
        // }
      } else if *tile != Kind(0) {
        let tile_gltf = ass.load("model/lab-tile.glb#Scene0");
        let mut transform = Transform::from_translation(loc);
        transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
        let entity = commands.spawn((SceneBundle {
            scene: tile_gltf,
            transform,
            ..Default::default()
        }, tracker)).id();
        entities.set(tracker, entity);
      }
    }

    // items
    for (loc, item) in auto.items.iter().enumerate() {
      let tracker = TrackedEntity::Item(auto_ndx, loc);
      let entity = entities.get(tracker);
      let loc = auto_loc + auto.ndx_to_loc(loc).as_vec2().extend(0.0);
      if let Some(entity) = entity {
        if let Ok(mut transform) = q.get_mut(entity) {
          transform.translation = loc;
        }
      } else if *item != Kind(0) {
        let tire_gltf = ass.load("model/wheelDefault.glb#Scene0");
        let mut transform = Transform::from_translation(loc);
        transform.rotate(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
        let entity = commands.spawn((SceneBundle {
            scene: tire_gltf,
            transform,
            ..Default::default()
        }, tracker)).id();
        entities.set(tracker, entity);
      }
    }
  }
}

#[derive(Resource)]
pub struct Entities {
  pub entities_map: HashMap<TrackedEntity, Entity>,
}

impl Entities {
  pub fn new() -> Self {
    Self {
      entities_map: HashMap::new(),
    }
  }

  pub fn get(&self, tracker: TrackedEntity) -> Option<Entity> {
    self.entities_map.get(&tracker).copied()
  }

  pub fn set(&mut self, tracker: TrackedEntity, entity: Entity) {
    self.entities_map.insert(tracker, entity);
  }
}

pub struct RS98EntitiesPlugin;

impl Plugin for RS98EntitiesPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Entities::new())
      .add_system(update_entities);
  }
}


