
use bevy::prelude::IVec2;
use conniver::{p};

use crate::model::{auto::{AutoNdx, Auto, auto_action_finished, auto_alive}, world::World, act::Action, dir::Dir, kind::{Kind, KindRole}};

use super::kind::Kinds;

#[test]
fn test_hiearchy() {
  let mut world = World::new_test();
  let space = AutoNdx(0);

  let earth = world.create_auto(Auto {
    kind: world.kinds.get("earth"),
    loc: IVec2::new(0, 0),
    parent: space,
    dim: IVec2::new(20, 20),
    ..Auto::default()
  });

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: IVec2::new(10, 10),
    parent: earth,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  let space_obj = world.get_auto(space);
  let earth_obj = world.get_auto(earth);
  let robo_obj = world.get_auto(robo);

  assert_eq!(space_obj.parent, space);
  assert_eq!(earth_obj.parent, space);
  assert_eq!(robo_obj.parent, earth);
  assert_eq!(space_obj.children, vec![earth]);
  assert_eq!(earth_obj.children, vec![robo]);
}

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
  world.set_auto_action(robo, Action::Step(Dir::North));
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

  world.set_auto_action(robo, Action::Step(Dir::North));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);

  world.set_auto_action(robo, Action::Pick(rock, world.kinds.nothing()));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find rock on ground.".to_string()));
  
  let new_loc = loc + Dir::North.to_ivec2();
  assert_eq!(world.get_auto(robo).loc, new_loc);
  world.set_item(space, new_loc, rock);
  world.set_auto_action(robo, Action::Pick(rock, world.kinds.nothing()));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(space, new_loc), world.kinds.nothing());
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), rock);

  world.set_auto_action(robo, Action::Step(Dir::South));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_auto(robo).loc, loc);
  assert_eq!(world.get_item(space, loc), rock);
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), rock);

  world.set_auto_action(robo, Action::Place(world.kinds.nothing()));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find empty slot on ground.".to_string()));
}

#[test]
fn test_pick_place_machine() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  world.set_all_tiles(space, world.kinds.get("grass"));
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

  world.set_item(machine_ndx, IVec2::new(0, 0), rock);
  world.set_auto_action(robo, Action::Place(machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find empty slot on machine.".to_string()));

  world.set_auto_action(robo, Action::Step(Dir::North));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_auto(robo).loc, IVec2::new(10, 11));

  world.set_auto_action(robo, Action::Place(machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find empty slot on machine.".to_string()));

}

#[test]
fn test_pick_filters() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let earth = world.create_auto(Auto {
    kind: world.kinds.get("earth"),
    loc: IVec2::new(0, 0),
    parent: space,
    dim: IVec2::new(20, 20),
    ..Auto::default()
  });
  world.set_all_tiles(earth, world.kinds.get("grass"));

  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  let machine = world.kinds.get("machine");
  let wildcard = Kind(1);
  let ground = Kind(0);

  let machine_ndx = world.create_auto(Auto {
    kind: machine,
    loc,
    parent: earth,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });
  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc,
    items: vec![rock],
    parent: earth,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  // Place into a kind that isn't there
  world.set_auto_action(robo, Action::Place(rock));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find empty slot on rock.".to_string()));

  // Cannot place on ground under machine
  world.set_auto_action(robo, Action::Place(ground));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find empty slot on ground.".to_string()));

  // Wildcard place
  world.set_auto_action(robo, Action::Place(wildcard));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), rock);
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), world.kinds.nothing());

  // Place when not holding anything
  world.set_auto_action(robo, Action::Place(wildcard));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Cannot place nothing.".to_string()));

  // Pick up nothing
  world.set_auto_action(robo, Action::Pick(ground, wildcard));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Cannot pick up nothing.".to_string()));

  // Pick up from ground which is empty
  world.set_auto_action(robo, Action::Pick(wildcard, ground));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find any on ground.".to_string()));

  // Successfully wildcard pick from machine
  world.set_auto_action(robo, Action::Pick(wildcard, wildcard));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), world.kinds.nothing());
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), rock);

  // Cannot place on ground under machine
  world.set_item(machine_ndx, IVec2::new(0, 0), rock);
  world.set_auto_action(robo, Action::Place(wildcard));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), Some("Could not find empty slot on any.".to_string()));

}

#[test]
fn test_produce() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let earth = world.create_auto(Auto {
    kind: world.kinds.get("earth"),
    loc: IVec2::new(0, 0),
    parent: space,
    ..Auto::default()
  });
  world.set_all_tiles(earth, world.kinds.get("grass"));

  let loc = IVec2::new(10, 10);
  let rock = world.kinds.get("rock");
  let machine = world.kinds.get("machine");
  let machine_ndx = world.create_auto(Auto {
    kind: machine,
    loc,
    parent: earth,
    ..Auto::default()
  });
  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc,
    items: vec![rock],
    parent: earth,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  assert_eq!(world.get_auto(machine_ndx).parent, earth);
  assert_eq!(world.get_auto(robo).parent, earth);
  assert_eq!(world.get_auto(earth).parent, space);
  assert_eq!(world.get_auto(earth).children, vec![machine_ndx, robo]);

  // Robo places rock in machine
  world.set_auto_action(robo, Action::Place(machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), rock);
  assert_eq!(world.get_item(robo, IVec2::new(0, 0)), world.kinds.nothing());

  // Machine produces thing
  world.set_auto_action(machine_ndx, Action::Produce);
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), world.kinds.get("thing"));

  // robo gets free rock, moves to machine's other slot
  world.set_item(robo, IVec2::new(0, 0), rock);
  world.set_auto_action(robo, Action::Step(Dir::East));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);

  // put rock in machine
  world.set_auto_action(robo, Action::Place(machine));
  world.update(2.0);
  assert_eq!(world.stall_message(robo), None);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(1, 0)), rock);
  assert_eq!(world.get_items(machine_ndx), vec![world.kinds.get("thing"), rock]);

  // machine produces widget
  world.set_auto_action(machine_ndx, Action::Produce);
  world.update(2.0);
  assert_eq!(world.get_item(machine_ndx, IVec2::new(0, 0)), world.kinds.get("widget"));
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
  assert_eq!(world.get_auto(robo2).flags.get(auto_alive), true);

  world.set_auto_action(robo1, Action::Fire(robo2));
  world.update(2.0);
  assert_eq!(world.stall_message(robo1), None);
  assert_eq!(world.get_auto(robo2).flags.get(auto_alive), false);

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
  assert_eq!(world.get_auto(robo3).flags.get(auto_alive), true);
}

#[test]
fn test_load_kinds() {
  let mut world = World::new_blank();
  
  world.kinds.set_by_val("nothing", p("(
    (traction 10)
  )"));

  world.kinds.set_by_val("missingno", p("(
    (traction 1)
  )"));

  world.kinds.set_by_val("robo", p("(
    (role auto)
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
  assert_eq!(robo_data.role, KindRole::Auto);
  
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

  world.kinds.set_by_val("missingno", p("(
    (dim 2 2)
  )"));
  let missingno = world.kinds.get("missingno");
  assert_eq!(missingno.0, 1);
  let missingno = world.kinds.get_data(missingno);
  assert_eq!(missingno.name, "missingno");
  assert_eq!(missingno.traction, 1);
  assert_eq!(missingno.item_dim, IVec2::new(2, 2));

  // rename
  world.kinds.set_by_val("robo", p("((name r1000))"));
  let new_robo = world.kinds.get("r1000");
  assert_eq!(new_robo, robo);
  let new_robo_data = world.kinds.get_data(new_robo);
  assert_eq!(new_robo_data.name, "r1000");
  assert_eq!(new_robo_data.traction, 0);
  assert_eq!(new_robo_data.item_dim, IVec2::new(1, 1));

  let missing = world.kinds.get("robo");
  assert_eq!(missing.0, 1);
}

#[test]
fn test_place_auto() {
  let mut world = World::new_test();
  world.kinds.set_by_val("robo", p("(
    (role auto)
    (traction 0)
    (dim 1 1)
    (scene \"model/r1000.glb#Scene0\")
  )"));
  let space = AutoNdx(0);
  let earth = world.create_auto(Auto {
    kind: world.kinds.get("earth"),
    loc: IVec2::new(0, 0),
    parent: space,
    dim: IVec2::new(10, 10),
    ..Auto::default()
  });

  let loc = IVec2::new(5, 5);
  let robo = world.kinds.get("robo");
  let nothing = world.kinds.get("nothing");
  let holder = world.create_auto(Auto {
    kind: robo,
    loc,
    parent: earth,
    ..Auto::default()
  });

  world.set_item(holder, IVec2::new(0, 0), world.kinds.get("robo"));
  assert_eq!(world.autos.len(), 3);

  world.set_auto_action(holder, Action::Place(nothing));
  world.update(2.0);
  assert_eq!(world.stall_message(holder), None);
  assert_eq!(world.get_item(holder, IVec2::new(0, 0)), nothing);
  assert_eq!(world.get_item(earth, loc), nothing);
  assert_eq!(world.autos.len(), 4);
  let new_auto = world.autos.last().unwrap();
  assert_eq!(new_auto.kind, robo);
  assert_eq!(new_auto.loc, loc);
  assert_eq!(new_auto.parent, earth);
  assert_eq!(new_auto.dim, IVec2::new(1, 1));
}

#[test]
fn test_kind_matching() {
  let nothing = Kind(0);
  let wildcard = Kind(1);
  let robo = Kind(2);

  assert!(nothing.matches(nothing));
  assert!(nothing.matches(wildcard));
  assert!(!nothing.matches(robo));
  assert!(wildcard.matches(nothing));
  assert!(wildcard.matches(wildcard));
  assert!(wildcard.matches(robo));
  assert!(!robo.matches(nothing));
  assert!(robo.matches(wildcard));
  assert!(robo.matches(robo));
}

#[test]
fn test_kind_names() {
  let kinds = Kinds::new_test();
  let nothing = kinds.get("nothing");
  let missingno = kinds.get("missingno");
  let robo = kinds.get("robo");

  assert_eq!(kinds.name(nothing), "nothing");
  assert_eq!(kinds.name(missingno), "missingno");
  assert_eq!(kinds.name(robo), "robo");

  assert_eq!(kinds.action_name(nothing), "ground");
  assert_eq!(kinds.action_name(missingno), "any");
  assert_eq!(kinds.action_name(robo), "robo");

  assert_eq!(kinds.name_list(&vec![nothing, missingno, robo]), "nothing missingno robo");
  assert_eq!(kinds.action_name_list(&vec![nothing, missingno, robo]), "ground any robo");

  assert_eq!(nothing, kinds.get("ground"));
  assert_eq!(missingno, kinds.get("any"));
}