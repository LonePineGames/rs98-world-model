use bevy::prelude::IVec2;

use crate::{auto::{AutoNdx, Auto}, world::World, act::Action, dir::Dir};

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

#[test]
fn test_pick_place() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  world.set_item(space, loc, rock);

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  world.set_auto_action(robo, Action::Pick(rock, world.kinds.nothing()));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(space, loc), world.kinds.nothing());
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), rock);

  world.set_auto_action(robo, Action::Place(world.kinds.nothing()));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(space, loc), rock);
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), world.kinds.nothing());

  world.set_auto_action(robo, Action::Move(Dir::North));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);

  world.set_auto_action(robo, Action::Pick(rock, world.kinds.nothing()));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find Kind(3) in Kind(0).".to_string()));

}
