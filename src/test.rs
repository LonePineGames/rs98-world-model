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
  assert_eq!(world.stall_message(robo), Some("Could not find Kind(3) on ground.".to_string()));

}

#[test]
fn test_pick_place_machine() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  let machine = world.kinds.get("machine");
  let machine_ndx = world.create_auto(Auto {
    kind: machine,
    loc,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });
  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc,
    items: vec![rock],
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  world.set_auto_action(robo, Action::Place(machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), rock);
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), world.kinds.nothing());

  world.set_auto_action(robo, Action::Pick(rock, machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), world.kinds.nothing());
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), rock);
}

#[test]
fn test_goto() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let start = IVec2::new(10, 10);
  let end = IVec2::new(20, 20);

  world.set_all_tiles(space, world.kinds.get("grass"));

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  world.set_auto_action(robo, Action::Goto(end));

  let mut steps = 0;
  while world.get_auto(robo).action != Action::Stop {
    world.update(2.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
  }

  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 21);
}

#[test]
fn test_goto_impeded() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let start = IVec2::new(10, 10);
  let end = IVec2::new(20, 20);

  world.set_all_tiles(space, world.kinds.get("grass"));

  for x in 0..40 {
    world.set_tile(space, IVec2::new(x, 15), world.kinds.get("wall"));
  }

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  world.set_auto_action(robo, Action::Goto(end));

  let mut steps = 0;
  while world.get_auto(robo).action != Action::Stop {
    world.update(2.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
  }

  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 43);
}

#[test]
fn test_produce() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  let machine = world.kinds.get("machine");
  let machine_ndx = world.create_auto(Auto {
    kind: machine,
    loc,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });
  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc,
    items: vec![rock],
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  world.set_auto_action(robo, Action::Place(machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), rock);
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), world.kinds.nothing());

  world.set_auto_action(machine_ndx, Action::Produce);
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), world.kinds.get("thing"));
}
