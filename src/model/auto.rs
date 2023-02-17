use bevy::prelude::IVec2;

use crate::model::{kind::Kind, act::Action};

use super::kind::Kinds;

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
  pub alive: bool,
  pub action_finished: bool,
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

  pub fn initalize(&self, kinds: &Kinds) -> Auto {
    let mut new = self.clone();
    let kind_data = kinds.get_data(new.kind);
    if new.dim.x == 0 || new.dim.y == 0 {
      new.dim = kind_data.item_dim;
    }
    new.alive = true;
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
