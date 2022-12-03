use bevy::prelude::IVec2;

use crate::{kind::Kind, act::Action};

#[derive(Clone, Default)]
pub struct Auto {
  pub kind: Kind,
  pub parent: AutoNdx,
  pub children: Vec<AutoNdx>,
  pub items: Vec<Kind>,
  pub tiles: Vec<Kind>,
  pub dim: IVec2,
  pub action: Action,
  pub loc: IVec2,
  pub action_time: f64,
  pub stall_message: Option<String>,
}

impl Auto {
  pub fn get_item(&self, loc: IVec2) -> Kind {
    let ndx = self.get_ndx(loc);
    if ndx >= 0 && ndx < self.items.len() as i32 {
      self.items[ndx as usize]
    } else {
      Kind(0)
    }
  }

  pub fn has_item(&self, loc: IVec2) -> bool {
    let ndx = self.get_ndx(loc);
    if ndx >= 0 && ndx < self.items.len() as i32 {
      self.items[ndx as usize].0 != 0
    } else {
      false
    }
  }

  pub fn set_item(&mut self, loc: IVec2, item: Kind) {
    let ndx = self.get_ndx(loc);
    if ndx >= 0 && ndx < self.items.len() as i32 {
      self.items[ndx as usize] = item;
    }
  }

  pub fn get_ndx(&self, loc: IVec2) -> i32 {
    loc.x + loc.y * self.dim.x
  }

  pub fn initalize(&self) -> Auto {
    let mut new = Auto {
      ..(*self).clone()
    };
    let num_items = (new.dim.x * new.dim.y) as usize;
    if new.items.len() < num_items {
      new.items.resize(num_items, Kind(0));
    }
    if new.tiles.len() < num_items {
      new.tiles.resize(num_items, Kind(0));
    }
    new
  }

  pub fn ndx_to_loc(&self, ndx: usize) -> IVec2 {
    IVec2::new(ndx as i32 % self.dim.x, ndx as i32 / self.dim.x)
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AutoNdx(pub usize);
