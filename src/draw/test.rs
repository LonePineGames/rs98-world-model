use bevy::prelude::*;

use crate::{model::{world::World, auto::{AutoNdx}}, draw::entities::{update_entities, TrackedEntity}, program::program::ProgramSpace};

use super::entities::Entities;

#[test]
fn test_entities() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc = IVec2::new(10, 10);
  let loc_ndx = world.get_auto(space).get_ndx(loc);
  let rock = world.kinds.get("rock");
  let grass = world.kinds.get("grass");
  world.set_item(space, loc, rock);

  let entities = Entities::new();

  let mut app = App::new();
  app.add_plugin(CorePlugin::default());
  app.add_plugin(AssetPlugin::default());
  app.insert_resource(world);
  app.insert_resource(entities);
  app.insert_resource(ProgramSpace::new(space));
  app.insert_resource(Time::default());
  app.add_system(update_entities);

  app.update();

  let entities = app.world.resource::<Entities>();
  assert_eq!(entities.entities_map.len(), 1);

  // get the scene, update the world, and check that the scene has changed
  let tracker = TrackedEntity::Item(space, loc_ndx as usize);
  let entity = entities.entities_map.get(&tracker).unwrap().clone();
  let scene = app.world.get::<Handle<Scene>>(entity).unwrap().clone();
  let mut world = app.world.resource_mut::<World>();
  world.set_item(space, loc, grass);

  // update twice to kill and respawn the entity
  app.update();
  app.update();

  let entities = app.world.resource::<Entities>();
  let entity = entities.entities_map.get(&tracker).unwrap().clone();
  let new_scene = app.world.get::<Handle<Scene>>(entity).unwrap().clone();
  assert_ne!(scene, new_scene);
}
