use crate::{auto::{Auto, AutoNdx}, kind::Kind};

pub struct World {
  pub autos: Vec<Auto>,
}

impl World {
    pub fn new_test() -> World {
      World {
        autos: vec![],
      }
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
}