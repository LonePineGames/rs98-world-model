use bevy::prelude::IVec2;
use conniver::p;

use crate::{model::{world::World, auto::{AutoNdx, Auto}, act::Action}, program::{program::ProgramSpace, event::get_event_handlers}};

#[test]
fn test_program_move() {
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

  let mut program = ProgramSpace::new();
  let event_handlers = get_event_handlers();
  program.set_program(robo, p("(goto 20 20)"));

  let mut steps = 0;
  while steps < 100 {
    program.update(1.0);
    program.process_events(&mut world, &event_handlers);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
    if world.get_auto(robo).loc == end && world.get_auto(robo).action == Action::Stop {
      break;
    }
  }

  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 21);
}

#[test]
fn test_input() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let start = IVec2::new(10, 10);
  let end = IVec2::new(9, 10);

  world.set_all_tiles(space, world.kinds.get("grass"));

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  let mut program = ProgramSpace::new();
  let event_handlers = get_event_handlers();
  program.interrupt(robo, p("(input-key a)"));
  
  let mut steps = 0;
  while steps < 100 {
    program.update( 1.0);
    program.process_events(&mut world, &event_handlers);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
    if world.get_auto(robo).loc == end && world.get_auto(robo).action == Action::Stop {
      break;
    }
  }
  
  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 1);
}
