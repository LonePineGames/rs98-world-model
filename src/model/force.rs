use std::collections::HashMap;

#[cfg(test)]
use bevy::prelude::IVec2;
#[cfg(test)]
use conniver::p;

#[cfg(test)]
use crate::{model::{world::World, auto::{AutoNdx, Auto}}, program::{test::run100, program::ProgramSpace}};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ForceNdx(pub usize);

pub struct Force {
  pub name: String,
}

pub struct Forces {
  pub forces: Vec<Force>,
  pub forces_by_name: HashMap<String, ForceNdx>,
}

impl Forces {
  pub fn new_blank() -> Forces {
    let mut result = Forces {
      forces: vec![],
      forces_by_name: HashMap::new(),
    };
    result.create(Force {
      name: "nature".to_string(),
    });
    result.create(Force {
      name: "forceless".to_string(),
    });
    result
  }

  pub fn create(&mut self, force: Force) -> ForceNdx {
    let ndx = ForceNdx(self.forces.len());
    self.forces_by_name.insert(force.name.clone(), ndx);
    self.forces.push(force);
    ndx
  }

  pub fn get(&self, name: &str) -> ForceNdx {
    if let Some(ndx) = self.forces_by_name.get(name) {
      *ndx
    } else {
      ForceNdx(1) // forceless
    }
  }
}

#[test]
fn test_force() {
  let mut world = World::new_test();
  let space = AutoNdx(0);

  let natural_force = world.forces.get("nature");
  let robo_force = world.forces.create(Force {
    name: "robo".to_string(),
  });

  let earth = world.create_auto(Auto {
    kind: world.kinds.get("earth"),
    loc: IVec2::new(0, 0),
    parent: space,
    force: natural_force,
    dim: IVec2::new(20, 20),
    ..Auto::default()
  });

  let robo = world.create_auto(Auto {
    kind: world.kinds.get("robo"),
    loc: IVec2::new(10, 10),
    parent: earth,
    force: robo_force,
    dim: IVec2::new(1, 1),
    ..Auto::default()
  });

  assert_eq!(world.get_auto(robo).force, robo_force);
  assert_eq!(world.get_auto(earth).force, natural_force);
}

#[test]
fn test_load_force() {
  let mut world = World::new_blank(); 
  let space = AutoNdx(0);
  let mut program = ProgramSpace::new(AutoNdx(0));

  program.interrupt(space, p("(do 
    (define-kind earth
      (traction 1)
    )
    (define-kind robo
      (traction 5)
    )
    (define-force nature)
    (define-force robo)

    (define earth-auto (create-auto 
      (kind earth)
      (loc 0 0)
      (parent 0)
      (force nature)
      (dim 50 50)
      (tile grass)
    ))
    (define player (create-auto 
      (kind robo)
      (loc 10 10)
      (parent earth-auto)
      (force robo)
      (dim 1 1)
    ))
    (access player)
    (set-item earth-auto 10 10 rock)
  )"));

  run100(&mut world, &mut program, space, -1);

  let earth = AutoNdx(1);
  let robo = AutoNdx(2);
  let nature_force = world.forces.get("nature");
  let robo_force = world.forces.get("robo");
  assert_eq!(world.get_auto(earth).force, nature_force);
  assert_eq!(world.get_auto(robo).force, robo_force);

}
