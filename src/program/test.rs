#![cfg(test)]

use bevy::prelude::IVec2;
use conniver::{val::p_all, p};

use crate::{model::{world::World, auto::{AutoNdx, Auto, auto_action_finished}, act::Action, kind::Kind, dir::Dir}, program::{program::ProgramSpace}};

pub fn run1(world: &mut World, program: &mut ProgramSpace, dur: f64) {
  program.update(dur);
  program.process_messages(world);
  world.update(dur);
}

pub fn run100(world: &mut World, program: &mut ProgramSpace, robo: AutoNdx, expected_steps: i32) {
  let mut steps = 0;
  while steps < 100 && (!program.idle(robo) || (world.get_auto(robo).action != Action::Stop && !world.get_auto(robo).flags.get(auto_action_finished))) {
    run1(world, program, 1.0);
    assert_eq!(world.stall_message(robo), None);
    steps += 1;
  }
  if expected_steps >= 0 {
    assert_eq!(steps, expected_steps);
  }
}

#[test]
fn test_program_goto() {
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
  run100(&mut world, &mut program, robo, 22);
  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
}

#[test]
fn test_multimove() {
  let mut world = World::new_test();
  let space = AutoNdx(0);
  let start = IVec2::new(10, 10);
  let end = IVec2::new(13, 14);

  world.set_all_tiles(space, world.kinds.get("grass"));

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: start,
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  let mut program = ProgramSpace::new(robo);

  program.set_program(robo, p("(move nnnneee)"));
  run100(&mut world, &mut program, robo, 15);
  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
}

#[test]
fn test_program_route() {
  let mut world = World::new_lab();
  let mut program = ProgramSpace::new(AutoNdx(0));

  let robo = AutoNdx(2);
  program.set_program(robo, p("(define my-route (route 1 21))"));
  run100(&mut world, &mut program, robo, -1);
  assert_eq!(program.get_var(robo, &"my-route".to_string()), p("\"eeeeeeeeeeeeeeeeeeennnnnnnnnnnnnnnnnnnnwwwwwwwwwwwwwwwwwwww\""));
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

  let mut program = ProgramSpace::new_lib_override(robo, &p_all("
    (define (input-key char)
      (cond 
        ((= char 'A) (set-program '(step w)))
        ((= char 'D) (set-program '(step e)))
        ((= char 'W) (set-program '(step n)))
        ((= char 'S) (set-program '(step s)))
        ((= char 'E) (if (= \"nothing\" (item-at me 0 0)) 
          (set-program '(pick))
          (set-program '(place))
        ))
        (else (set-program '(stop)))
      )
    )

    (define (input-mouse x y)
      (set-program ('goto x y))
    )"));

  program.interrupt(robo, p("(input-key A)"));
  run100(&mut world, &mut program, robo, 2);
  assert_eq!(world.get_auto(robo).loc, end);
  assert_eq!(world.get_auto(robo).action, Action::Stop);

  program.interrupt(robo, p("(input-key D)"));
  run100(&mut world, &mut program, robo, 2);
  assert_eq!(world.get_auto(robo).loc, start);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  
  program.interrupt(robo, p("(input-mouse 20 20)"));
  run100(&mut world, &mut program, robo, 22);
  assert_eq!(world.get_auto(robo).loc, elsewhere);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
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

  let mut program = ProgramSpace::new_lib_override(robo, &p_all("
    (define (input-key char)
      (cond 
        ((= char 'A) (set-program '(move w)))
        ((= char 'D) (set-program '(move e)))
        ((= char 'W) (set-program '(move n)))
        ((= char 'S) (set-program '(move s)))
        ((= char 'E) (if (= \"nothing\" (item-at me 0 0)) 
          (set-program '(pick))
          (set-program '(place))
        ))
        (else (set-program '(stop)))
      )
    )"));
  
  program.interrupt(robo, p("(input-key E)"));
  run100(&mut world, &mut program, robo, 3);
  assert_eq!(world.get_item(robo, hand), rock);
  assert_eq!(world.get_item(space, start), Kind(0));
  assert_eq!(world.get_auto(robo).action, Action::Stop);

  program.interrupt(robo, p("(input-key E)"));
  run100(&mut world, &mut program, robo, 3);
  assert_eq!(world.get_item(robo, hand), Kind(0));
  assert_eq!(world.get_auto(robo).loc, start);
  assert_eq!(world.get_auto(robo).action, Action::Stop);
  assert_eq!(world.get_item(space, start), rock);
}

#[test]
fn test_load() {
  let mut world = World::new_blank(); 
  let space = AutoNdx(0);
  let mut program = ProgramSpace::new(AutoNdx(0));

  program.interrupt(space, p("(do 
    (define-kind rock
      (traction 1)
    )
    (define-kind grass
      (traction 1)
    )
    (define-kind earth
      (traction 1)
    )
    (define-kind robo
      (traction 5)
      (program '(loop (move e) (move w)))
    )
    (define earth-auto (create-auto 
      (kind earth)
      (loc 0 0)
      (parent 0)
      (dim 50 50)
      (tile grass)
    ))
    (define player (create-auto 
      (kind robo)
      (loc 10 10)
      (parent earth-auto)
      (dim 1 1)
    ))
    (access player)
    (set-item earth-auto 10 10 rock)
  )"));

  run100(&mut world, &mut program, space, -1);
  
  let rock = world.kinds.get("rock");
  let rock_data = world.kinds.get_data(rock);
  assert_eq!(rock_data.name, "rock");
  assert_eq!(rock_data.traction, 1);

  let earth = AutoNdx(1);
  let earth_data = world.get_auto(earth);
  assert_eq!(earth_data.kind, world.kinds.get("earth"));
  assert_eq!(earth_data.loc, IVec2::new(0, 0));
  assert_eq!(earth_data.parent, space);
  assert_eq!(earth_data.dim, IVec2::new(50, 50));
  assert_eq!(world.get_tile(earth, IVec2::new(0, 0)), world.kinds.get("grass"));
  assert_eq!(world.get_item(earth, IVec2::new(10, 10)), rock);

  let robo_auto = AutoNdx(2);
  run1(&mut world, &mut program, 0.1);

  let robo = world.kinds.get("robo");
  let robo_data = world.get_auto(robo_auto);
  assert_eq!(robo_data.kind, robo);
  assert_eq!(robo_data.parent, earth);
  assert_eq!(program.access, robo_auto);

  // verify that the robo's program is running
  assert_eq!(robo_data.loc, IVec2::new(10, 10));
  assert_eq!(robo_data.action, Action::Stop);
  run1(&mut world, &mut program, 0.1);
  let robo_data = world.get_auto(robo_auto);
  assert_eq!(robo_data.loc, IVec2::new(10, 10));
  assert_eq!(robo_data.action, Action::Step(Dir::East));
  run1(&mut world, &mut program, 1.0);
  run1(&mut world, &mut program, 0.1);
  let robo_data = world.get_auto(robo_auto);
  assert_eq!(robo_data.loc, IVec2::new(11, 10));
  assert_eq!(robo_data.action, Action::Stop);

}

#[test]
fn test_define_pattern() {
  let mut world = World::new_blank(); 
  let space = AutoNdx(0);
  let mut program = ProgramSpace::new(AutoNdx(0));

  program.interrupt(space, p("(do 
    (define-kind rock
      (traction 1)
    )
    (define-kind grass
      (traction 1)
    )
    (define-kind robo
      (traction 1)
    )
    (define-pattern
      (for robo)
      (in (nothing nothing nothing grass grass grass nothing nothing nothing))
      (out (nothing nothing nothing grass rock grass nothing nothing nothing))
    )
  )"));

  run100(&mut world, &mut program, space, -1);
  assert_eq!(world.patterns.len(), 1);
  
  let rock = world.kinds.get("rock");
  let robo = world.kinds.get("robo");
  let nothing = world.kinds.get("nothing");
  let grass = world.kinds.get("grass");

  let pattern = world.patterns.get(robo, &vec![nothing, nothing, nothing, grass, grass, grass, nothing, nothing, nothing]);
  if let Some(pattern) = pattern {
    assert_eq!(pattern.output, vec![
      nothing, nothing, nothing,
      grass, rock, grass,
      nothing, nothing, nothing,
    ]);
  } else {
    assert!(false);
  }
}

#[test]
fn test_access() {
  let mut world = World::new_test(); 
  let space = AutoNdx(0);
  let mut program = ProgramSpace::new(AutoNdx(0));

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: IVec2::new(0, 0),
    parent: space,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });
  assert_eq!(robo, AutoNdx(1));

  assert_eq!(program.access, space);
  program.interrupt(space, p("(access 1)"));
  run100(&mut world, &mut program, space, -1);
  assert_eq!(program.access, robo);
}

#[test]
fn test_automine() {
  let mut world = World::new_blank(); 
  let space = AutoNdx(0);
  let mut program = ProgramSpace::new(AutoNdx(0));

  program.interrupt(space, p("(do 
    (define-kind rock
      (traction 1)
    )
    (define-kind grass
      (role tile)
      (traction 1)
    )
    (define-kind earth
      (role auto)
      (traction 1)
    )
    (define-kind robo
      (role auto)
      (traction 5)
    )
    (define-kind automine
      (role auto)
      (dim 1 1)
      (program '(loop (set-item me 0 0 rock)))
    )

    (define earth-auto (create-auto 
      (kind earth)
      (loc 0 0)
      (parent 0)
      (dim 50 50)
      (tile grass)
    ))
    
    (create-auto 
      (kind robo)
      (loc 10 10)
      (parent earth-auto)
      (dim 1 1)
    )
    (create-auto 
      (kind automine)
      (loc 10 10)
      (parent earth-auto)
      (dim 1 1)
    )
  )"));
  run100(&mut world, &mut program, space, -1);

  assert_eq!(world.autos.len(), 4);
  let automine_auto = AutoNdx(3);
  let robo_auto = AutoNdx(2);
  let automine = world.kinds.get("automine");
  let rock = world.kinds.get("rock");
  let nothing = world.kinds.get("nothing");

  assert_eq!(world.get_auto(automine_auto).kind, automine);
  assert_eq!(world.get_auto(robo_auto).kind, world.kinds.get("robo"));
  assert_eq!(world.get_auto(automine_auto).loc, IVec2::new(10, 10));
  assert_eq!(world.get_auto(robo_auto).loc, IVec2::new(10, 10));

  assert_eq!(world.get_item(automine_auto, IVec2::new(0, 0)), rock);
  assert_eq!(world.get_item(robo_auto, IVec2::new(0, 0)), nothing);

  world.set_auto_action(robo_auto, Action::Pick(rock, automine));
  run100(&mut world, &mut program, robo_auto, 1);
  assert_eq!(world.get_item(automine_auto, IVec2::new(0, 0)), nothing);
  assert_eq!(world.get_item(robo_auto, IVec2::new(0, 0)), rock);

  run1(&mut world, &mut program, 1.0);
  run1(&mut world, &mut program, 1.0);
  assert_eq!(world.get_item(automine_auto, IVec2::new(0, 0)), rock);

  // have the robo place another automine, then test it (verify program init)
  world.set_item(robo_auto, IVec2::new(0, 0), automine);
  world.get_auto_mut(robo_auto).loc = IVec2::new(20, 20); // holy teleporation batman
  world.set_auto_action(robo_auto, Action::Place(nothing));
  run100(&mut world, &mut program, robo_auto, 1);
  assert_eq!(world.autos.len(), 5);
  let automine_auto2 = AutoNdx(4);
  assert_eq!(world.get_auto(automine_auto2).kind, world.kinds.get("automine"));
  assert_eq!(world.get_auto(automine_auto2).loc, IVec2::new(20, 20));
  run1(&mut world, &mut program, 1.0);
  run1(&mut world, &mut program, 1.0);
  assert_eq!(world.get_item(automine_auto2, IVec2::new(0, 0)), rock);
}

// #[test]
// fn test_mass_produce() {
//   let mut world = World::new_blank();
//   let space = AutoNdx(0);
//   let mut program = ProgramSpace::new(space);

//   program.interrupt(space, p("(do 
//     (define-kind rock
//       (traction 1)
//     )
//     (define-kind grass
//       (traction 1)
//     )
//     (define-kind earth
//       (traction 1)
//     )
//     (define-kind robo
//       (traction 1)
//     )
//     (define earth-auto (create-auto 
//       (kind earth)
//       (loc 0 0)
//       (parent 0)
//       (dim 50 50)
//       (tile grass)
//     ))
//     (define player (create-auto 
//       (kind robo)
//       (loc 10 10)
//       (parent earth-auto)
//       (dim 1 1)
//     ))
//     (access player)
//     (set-item earth-auto 10 10 rock)
//   )"));
// }
