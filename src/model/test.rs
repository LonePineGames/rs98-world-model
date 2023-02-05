
use bevy::prelude::IVec2;
use conniver::{State, p};

use crate::model::{auto::{AutoNdx, Auto}, world::World, act::Action, dir::Dir};

#[test]
fn test_move_impeded() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let start = IVec2::new(10, 10);
  let end = IVec2::new(10, 11);
  world.set_tile(space, end, world.kinds.get("wall"));
  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });
  world.set_auto_action(robo, Action::Move(Dir::North));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not move to (10,11): robo cannot cross wall.".to_string()));
}

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
  world.set_all_tiles(space, world.kinds.get("grass"));
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

#[test]
fn test_fire() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let loc1 = IVec2::new(10, 10);
  let robo1 = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: loc1,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  let loc2 = IVec2::new(10, 12);
  let robo2 = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: loc2,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });
  assert_eq!(world.get_auto(robo2).alive, true);

  world.set_auto_action(robo1, Action::Fire(robo2));
  world.update(2.0);
  assert_eq!(world.stall_message(robo1), None);
  assert_eq!(world.get_auto(robo2).alive, false);

  // test range limits
  let loc3 = IVec2::new(10, 20);
  let robo3 = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: loc3,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  world.set_auto_action(robo1, Action::Fire(robo3));
  world.update(2.0);

  assert_eq!(world.stall_message(robo1), Some("Target out of range.".to_string()));
  assert_eq!(world.get_auto(robo3).alive, true);
}

#[test]
fn test_load_kinds() {
  let mut world = World::new_blank();
  
  world.kinds.set_by_val(p("(
    (name nothing)
    (traction 10)
  )"));

  world.kinds.set_by_val(p("(
    (name missingno)
    (traction 1)
  )"));

  world.kinds.set_by_val(p("(
    (name robo)
    (traction 0)
    (dim 1 1)
    (scene \"model/r1000.glb#Scene0\")
  )"));

  let nothing = world.kinds.get("nothing");
  assert_eq!(nothing.0, 0);
  let nothing = world.kinds.get_data(nothing);
  assert_eq!(nothing.name, "nothing");
  assert_eq!(nothing.traction, 10);
  assert_eq!(nothing.item_dim, IVec2::new(0, 0));
  assert_eq!(nothing.scene, "");

  let missingno = world.kinds.get("missingno");
  assert_eq!(missingno.0, 1);

  let robo = world.kinds.get("robo");
  let robo_data = world.kinds.get_data(robo);
  assert_eq!(robo_data.name, "robo");
  assert_eq!(robo_data.traction, 0);
  assert_eq!(robo_data.item_dim, IVec2::new(1, 1));
  assert_eq!(robo_data.scene, "model/r1000.glb#Scene0");
  
  let robot = world.create_auto(Auto {
    kind: robo,
    loc: IVec2::new(0, 0),
    parent: AutoNdx(0),
    ..Auto::default()
  });

  let robo_data = world.get_auto(robot);
  assert_eq!(robo_data.kind, robo);
  assert_eq!(robo_data.loc, IVec2::new(0, 0));
  assert_eq!(robo_data.parent, AutoNdx(0));
  assert_eq!(robo_data.dim, IVec2::new(1, 1));

  world.kinds.set_by_val(p("(
    (name missingno)
    (dim 2 2)
  )"));
  let missingno = world.kinds.get("missingno");
  assert_eq!(missingno.0, 1);
  let missingno = world.kinds.get_data(missingno);
  assert_eq!(missingno.name, "missingno");
  assert_eq!(missingno.traction, 1);
  assert_eq!(missingno.item_dim, IVec2::new(2, 2));

}