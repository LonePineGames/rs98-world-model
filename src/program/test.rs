use bevy::prelude::IVec2;
use conniver::p;

use crate::{model::{world::World, auto::{AutoNdx, Auto}, act::Action, kind::Kind}, program::{program::ProgramSpace, event::get_event_handlers}};

fn run100(world: &mut World, program: &mut ProgramSpace, robo: AutoNdx, expected_steps: usize) {
  let mut steps = 0;
  while steps < 100 && !program.idle(robo) {
    program.update(1.0);
    program.process_events(world);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
  }
  assert_eq!(steps, expected_steps);
}

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

  let mut program = ProgramSpace::new(robo);
  program.set_program(robo, p("(goto 20 20)"));

  let mut steps = 0;
  while steps < 100 {
    program.update(1.0);
    program.process_events(&mut world);
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
  let elsewhere = IVec2::new(20, 20);

  world.set_all_tiles(space, world.kinds.get("grass"));

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  let mut program = ProgramSpace::new(robo);
  program.interrupt(robo, p("(input-key A)"));
  
  let mut steps = 0;
  while steps < 100 {
    program.update( 1.0);
    program.process_events(&mut world);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
    if world.get_auto(robo).loc == end && world.get_auto(robo).action == Action::Stop && program.idle(robo) {
      break;
    }
  }
  
  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 1);

  program.interrupt(robo, p("(input-key E)"));
  
  while steps < 100 {
    program.update( 1.0);
    program.process_events(&mut world);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
    if world.get_auto(robo).loc == start && world.get_auto(robo).action == Action::Stop && program.idle(robo) {
      break;
    }
  }
  
  assert_eq!(world.get_auto(robo).loc, start);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 2);
  
  program.interrupt(robo, p("(input-mouse 20 20)"));
  
  while steps < 100 {
    program.update( 1.0);
    program.process_events(&mut world);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
    if world.get_auto(robo).loc == elsewhere && world.get_auto(robo).action == Action::Stop && program.idle(robo) {
      break;
    }
  }
  
  assert_eq!(world.get_auto(robo).loc, elsewhere);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 23);
}

#[test]
fn test_input_pick_place() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let start = IVec2::new(10, 10);
  let hand = IVec2::new(0, 0);
  let rock = world.kinds.get("rock");

  world.set_all_tiles(space, world.kinds.get("grass"));
  world.set_item(space, start, rock);
  assert_eq!(world.get_item(space, start), rock);

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  let mut program = ProgramSpace::new(robo);
  program.interrupt(robo, p("(input-key Period)"));
  
  let mut steps = 0;
  while steps < 100 && !program.idle(robo) {
    program.update( 1.0);
    program.process_events(&mut world);
    program.process_messages(&mut world);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
  }
  
  assert_eq!(world.get_item(robo, hand), rock);
  assert_eq!(world.get_item(space, start), Kind(0));
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(steps, 2);

  program.interrupt(robo, p("(input-key Period)"));
  
  while steps < 100 {
    program.update( 1.0);
    program.process_events(&mut world);
    program.process_messages(&mut world);
    world.update(1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
    if world.get_item(robo, hand) == Kind(0) && world.get_auto(robo).action == Action::Stop && program.idle(robo) {
      break;
    }
  }
  
  assert_eq!(world.get_item(robo, hand), Kind(0));
  assert_eq!(world.get_auto(robo).loc, start);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(world.get_item(space, start), rock);
  assert_eq!(steps, 4);
}