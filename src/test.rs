use bevy::prelude::IVec2;

use crate::{kind::Kinds, auto::{Auto, AutoNdx}, world::World};

#[test]
fn test_items_on_ground() {
  let kinds = Kinds::new_test();
  let mut world = World::new_test();
  let space_ndx = world.create_auto(Auto {
    kind: kinds.get("earth"),
    parent: AutoNdx(0),
    children: vec![],
    items: vec![],
    dim: IVec2::new(100, 100),
  });
  let space = world.get_auto_mut(space_ndx);
  let loc = IVec2::new(10, 10);
  let rock = kinds.get("rock");
  space.set_item(loc, rock);
  assert_eq!(space.get_item(loc), rock);
  space.set_item(loc, kinds.nothing());
  assert_eq!(space.has_item(loc), false);
}
