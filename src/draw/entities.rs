use std::collections::HashMap;

use bevy::{prelude::*, scene::SceneBundle};

use crate::model::{world::World, auto::AutoNdx, kind::Kind};

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TrackedEntity {
  Auto(AutoNdx),
  Tile(AutoNdx, usize),
  Item(AutoNdx, usize),
}

pub fn update_entities(
  mut commands: Commands,
  world: Res<World>,
  mut entities: ResMut<Entities>,
  ass: Res<AssetServer>,
  q: Query<&mut Transform, With<TrackedEntity>>,
) {
  for (auto_ndx, auto) in world.autos.iter().enumerate() {
    let auto_ndx = AutoNdx(auto_ndx);
    for (loc, item) in auto.items.iter().enumerate() {
      let tracker = TrackedEntity::Item(auto_ndx, loc);
      let entity = entities.get(tracker);
      if let Some(entity) = entity {

      } else if *item != Kind(0) {
        let tire_gltf = ass.load("model/wheelDefault.glb#Scene0");
        let mut transform = Transform::from_translation(auto.loc.as_vec2().extend(0.0));
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
      .add_system(update_entities);
  }
}


