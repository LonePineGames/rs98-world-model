use bevy::prelude::Entity;

use crate::model::world::World;

pub struct Entities {
}

impl Entities {
  pub fn new(world: World) -> Self {
    Self {
    }
  }

  pub fn count(&self) -> usize {
    20
  }
}
