use bevy::prelude::IVec2;

use crate::{auto::{Auto, AutoNdx}, kind::{Kind, Kinds}};

pub struct World {
  pub autos: Vec<Auto>,
  pub kinds: Kinds,
}

impl World {
  pub fn new_test() -> World {
    let mut world = World {
      kinds: Kinds::new_test(),
      autos: vec![],
    };
    world.create_auto(Auto {
      kind: world.kinds.get("earth"),
      parent: AutoNdx(0),
      children: vec![],
      items: vec![],
      dim: IVec2::new(100, 100),
    });
    world
  }

  pub fn create_auto(&mut self, new: Auto) -> AutoNdx {
    let mut new = new;
    let num_items = (new.dim.x * new.dim.y) as usize;
    if new.items.len() < num_items {
      new.items.resize(num_items, Kind(0));
    }
    self.autos.push(new);
    AutoNdx(self.autos.len() - 1)
  }

  pub fn get_auto_mut(&mut self, space_ndx: AutoNdx) -> &mut Auto {
    &mut self.autos[space_ndx.0]
  }

  pub fn set_item(&mut self, auto: AutoNdx, loc: bevy::prelude::IVec2, item: Kind) {
    let auto = self.get_auto_mut(auto);
    auto.set_item(loc, item);
  }

  pub fn get_item(&self, auto: AutoNdx, loc: bevy::prelude::IVec2) -> Kind {
    let auto = &self.autos[auto.0];
    auto.get_item(loc)
  }

  pub fn has_item(&self, auto: AutoNdx, loc: bevy::prelude::IVec2) -> bool {
    let auto = &self.autos[auto.0];
    auto.has_item(loc)
  }
}