use std::{collections::{HashMap, HashSet}, f32::consts::PI};

use bevy::{prelude::*, scene::SceneBundle};

use crate::{model::{world::World, auto::AutoNdx, kind::Kind}, program::program::ProgramSpace};

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TrackedEntity {
  Auto(AutoNdx),
  Tile(AutoNdx, usize),
  Item(AutoNdx, usize),
}

#[derive(Component, Debug)]
pub struct Physics {
  pub vel: Vec3,
}

pub fn update_entity(
  tracker: TrackedEntity,
  loc: Vec3,
  kind: Kind,
  commands: &mut Commands,
  world: &Res<World>,
  entities: &mut ResMut<Entities>,
  ass: &Res<AssetServer>,
  q: &mut Query<(&mut Transform, &mut Physics), With<TrackedEntity>>,
  time: &Res<Time>,
) {
  let entity = entities.get(tracker);
  if let Some(entity) = entity {
    if kind == Kind(0) {
      commands.entity(entity).despawn_recursive();
      entities.entities_map.remove(&tracker);
    } else if let Ok((mut transform, mut physics)) = q.get_mut(entity) {

      let diff = loc - transform.translation;
      physics.vel = mix(physics.vel, diff*0.5, 0.2);
      if physics.vel.length() > 0.1 {
        transform.translation += physics.vel * time.delta_seconds();
        transform.rotation = Quat::from_rotation_x(PI/2.0);
        transform.rotate(Quat::from_rotation_z((-physics.vel.x).atan2(physics.vel.y)));
      }
    }
  } else if kind != Kind(0) {
    let data = world.kinds.get_data(kind);
    let scene = ass.load(data.scene.clone());
    let mut transform = Transform::from_translation(loc);
    transform.rotate(Quat::from_rotation_x(PI/2.0));
    let entity = commands.spawn((
      SceneBundle {
        scene,
        transform,
        ..Default::default()
      }, 
      tracker,
      Physics {
        vel: Vec3::ZERO,
      }
    )).id();
    entities.set(tracker, entity);
  }
}

fn mix(vel: Vec3, diff: Vec3, arg: f32) -> Vec3 {
  let diff = diff * arg;
  let vel = vel * (1.0 - arg);
  vel + diff
}

pub fn update_entities(
  mut commands: Commands,
  world: Res<World>,
  program: Res<ProgramSpace>,
  mut entities: ResMut<Entities>,
  ass: Res<AssetServer>,
  mut q: Query<(&mut Transform, &mut Physics), With<TrackedEntity>>,
  time: Res<Time>,
) {

  let mut to_update = HashSet::new();

  let access = program.access;
  let parent_ndx = world.get_auto(access).parent;
  let parent = world.get_auto(parent_ndx);
  //to_update.insert(TrackedEntity::Auto(parent_ndx));

  for auto_ndx in parent.children.iter() {
    to_update.insert(TrackedEntity::Auto(*auto_ndx));
    let auto = world.get_auto(*auto_ndx);
    for i in 0..auto.items.len() {
      to_update.insert(TrackedEntity::Item(*auto_ndx, i));
    }
  }

  for (i, item) in parent.items.iter().enumerate() {
    if *item != Kind(0) {
      to_update.insert(TrackedEntity::Item(parent_ndx, i));
    }
  }
  
  let num_tiles = parent.tiles.len();
  let num_tiles_to_update = num_tiles.min(20);
  for _ in 0..num_tiles_to_update {
    let loc = entities.tile_update;
    to_update.insert(TrackedEntity::Tile(parent_ndx, loc));
    to_update.insert(TrackedEntity::Item(parent_ndx, loc));
    entities.tile_update = (entities.tile_update + 1) % num_tiles;
  }

  let existing = entities.entities_map.keys().cloned().collect::<HashSet<_>>();
  for tracker in existing.iter() {
    if !to_update.contains(tracker) {
      let keep = if let TrackedEntity::Tile(tile_parent, _) = tracker {
        tile_parent == &parent_ndx
      } else {
        false
      };
      if !keep {
        let entity = entities.get(*tracker);
        if let Some(entity) = entity {
          commands.entity(entity).despawn_recursive();
          entities.entities_map.remove(tracker);
        }
      }
    }
  }

  //println!("update calls: {}, {:?}", to_update.len(), time.delta());

  for tracker in to_update {
    match tracker {
      TrackedEntity::Auto(auto_ndx) => {
        let auto = world.get_auto(auto_ndx);
        let auto_loc = auto.loc.as_vec2().extend(0.0);
        //println!("auto: {:?} {:?} {:?}/{:?}", auto.kind, auto_loc, auto_ndx, access);
        update_entity(tracker, auto_loc, auto.kind, &mut commands, &world, &mut entities, &ass, &mut q, &time);
      },
      TrackedEntity::Tile(auto_ndx, loc) => {
        let auto = world.get_auto(auto_ndx);
        let auto_loc = auto.loc.as_vec2().extend(0.0);
        let tile = auto.tiles[loc];
        let loc = auto_loc + auto.ndx_to_loc(loc).as_vec2().extend(0.0);
        update_entity(tracker, loc, tile, &mut commands, &world, &mut entities, &ass, &mut q, &time);
      },
      TrackedEntity::Item(auto_ndx, loc) => {
        let auto = world.get_auto(auto_ndx);
        let auto_loc = auto.loc.as_vec2().extend(0.0);
        let item = auto.items[loc];
        let loc = auto_loc + auto.ndx_to_loc(loc).as_vec2().extend(0.0);
        update_entity(tracker, loc, item, &mut commands, &world, &mut entities, &ass, &mut q, &time);
      },
    }
  }


  /*let mut update_calls = 0;
  for (auto_ndx, auto) in world.autos.iter().enumerate() {
    let auto_ndx = AutoNdx(auto_ndx);
    let num_tiles = auto.tiles.len();

    let auto_tracker = TrackedEntity::Auto(auto_ndx);
    let auto_loc = auto.loc.as_vec2().extend(0.0);
    update_entity(auto_tracker, auto_loc, auto.kind, &mut commands, &world, &mut entities, &ass, &mut q, &time);
    update_calls += 1;

    // tiles
    if num_tiles > 0 {
      let loc = entities.tile_update;
      let tile = auto.tiles[loc];
      let tracker = TrackedEntity::Tile(auto_ndx, loc);
      let loc = auto_loc + auto.ndx_to_loc(loc).as_vec2().extend(0.0);
      update_entity(tracker, loc, tile, &mut commands, &world, &mut entities, &ass, &mut q, &time);
      update_calls += 1;
    }

    // items
    for (loc, item) in auto.items.iter().enumerate() {
      let tracker = TrackedEntity::Item(auto_ndx, loc);
      let loc = auto_loc + auto.ndx_to_loc(loc).as_vec2().extend(0.0);
      update_entity(tracker, loc, *item, &mut commands, &world, &mut entities, &ass, &mut q, &time);
      update_calls += 1;
    }

    entities.tile_update = if num_tiles == 0 {
      0
    } else {
      (entities.tile_update + 1) % num_tiles
    };
  }
  println!("update calls: {}, {:?}", update_calls, time.delta());*/
}

#[derive(Resource)]
pub struct Entities {
  pub entities_map: HashMap<TrackedEntity, Entity>,
  pub tile_update: usize,
}

impl Entities {
  pub fn new() -> Self {
    Self {
      entities_map: HashMap::new(),
      tile_update: 0,
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


