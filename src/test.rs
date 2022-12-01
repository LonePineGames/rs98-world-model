use bevy::prelude::IVec2;

use crate::{auto::{AutoNdx}, world::World};

#[test]
fn test_items_on_ground() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  world.set_item(space, loc, rock);
  assert_eq!(world.get_item(space, loc), rock);
  world.set_item(space, loc, world.kinds.nothing());
  assert_eq!(world.has_item(space, loc), false);
}
