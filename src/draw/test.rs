use bevy::prelude::*;

use crate::{model::{world::World, auto::{AutoNdx}}, draw::entities::{update_entities}};

use super::entities::Entities;

#[test]
fn test_entities() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  world.set_item(space, loc, rock);

  let entities = Entities::new();

  let mut app = App::new();
  app.add_plugin(CorePlugin::default());
  app.add_plugin(AssetPlugin::default());
  app.insert_resource(world);
  app.insert_resource(entities);
  app.add_system(update_entities);

  app.update();

  let entities = app.world.resource::<Entities>();
  assert_eq!(entities.entities_map.len(), 1);

}
