
use bevy::prelude::IVec2;

use crate::{auto::{Auto, AutoNdx}, kind::{Kind, Kinds}, act::Action};

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
      dim: IVec2::new(100, 100),
      ..Auto::default()
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
    let auto = self.get_auto(auto);
    auto.get_item(loc)
  }

  pub fn has_item(&self, auto: AutoNdx, loc: bevy::prelude::IVec2) -> bool {
    let auto = self.get_auto(auto);
    auto.has_item(loc)
  }

  pub fn set_auto_action(&mut self, auto: AutoNdx, action: Action) {
    let auto = self.get_auto_mut(auto);
    auto.action = action;
  }

  pub fn stall_message(&self, auto: AutoNdx) -> Option<String> {
    let auto = self.get_auto(auto);
    auto.stall_message.clone()
  }

  pub fn get_auto(&self, auto: AutoNdx) -> &Auto {
    &self.autos[auto.0]
  }

  pub fn auto_ndxes(&self) -> Vec<AutoNdx> {
    (0..self.autos.len()).map(|ndx| AutoNdx(ndx)).collect()
  }

  pub fn update(&mut self, dur: f64) {
    for auto in self.auto_ndxes() {
      self.update_auto(auto, dur);
    }
  }

  pub fn update_auto(&mut self, ndx: AutoNdx, dur: f64) {
    let auto = self.get_auto_mut(ndx);
    auto.action_time += dur;
    if auto.action_time >= 1.0 {
      auto.action_time = 0.0;
      let action = auto.action;
      let stall_message = action.act(self, ndx);
      let auto = self.get_auto_mut(ndx);
      if stall_message.is_none() {
        auto.action = Action::Stop;
      }
      auto.stall_message = stall_message;
    }
  }
}