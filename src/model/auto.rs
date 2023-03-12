#![allow(non_upper_case_globals)] 
#![allow(non_upper_case_globals)] // no way! who wants the code to scream at them? not me.

use bevy::prelude::IVec2;
use conniver::{read_object, read_ivec2, object::read_string, Val};

use crate::model::{kind::Kind, act::Action};

use super::{kind::Kinds, force::ForceNdx, world::World, bitfield::{BitField, BFNdx}};

#[derive(Clone, Default, Debug)]
pub struct Auto {
  pub kind: Kind,
  pub parent: AutoNdx,
  pub children: Vec<AutoNdx>,
  pub force: ForceNdx,
  pub items: Vec<Kind>,
  pub tiles: Vec<Kind>,
  pub dim: IVec2,
  pub action: Action,
  pub loc: IVec2,
  pub action_time: f64,
  pub stall_message: Option<String>,
  pub flags: BitField,
}

#[allow(dead_code)]
pub const auto_exists          : BFNdx = BFNdx::new(0);
pub const auto_alive           : BFNdx = BFNdx::new(1);
pub const auto_action_finished : BFNdx = BFNdx::new(2);

impl Auto {
  pub fn get_item(&self, loc: IVec2) -> Kind {
    let ndx = self.get_ndx(loc);
    if ndx >= 0 && ndx < self.items.len() as i32 {
      self.items[ndx as usize]
    } else {
      Kind(0)
    }
  }

  #[cfg(test)]
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
    new.flags.set(auto_alive, true);
    let num_items = (new.dim.x * new.dim.y) as usize;
    if new.items.len() < num_items {
      new.items.resize(num_items, Kind(0));
    }
    if new.tiles.len() < num_items {
      new.tiles.resize(num_items, Kind(0));
    }
    new
  }

  pub fn from_val(val: Val, world: &World) -> Auto {
    let mut auto = Auto::default();
    let mut tile_kind = Kind(0);
    read_object(&val, |key, val| {
      match key {
        "kind" => auto.kind = world.kinds.get(&read_string(val)),
        "parent" => if let Val::Num(i) = val {
          auto.parent = AutoNdx(*i as usize);
        } else {
          println!("bad parent: {val:?}");
        },
        "force" => auto.force = world.forces.get(&read_string(val)),
        "tile" => tile_kind = world.kinds.get(&read_string(val)),
        "dim" => read_ivec2(val, |x, y| {
          auto.dim = IVec2::new(x, y);
        }, || {
          println!("bad dim: {val:?}");
        }),
        "loc" => read_ivec2(val, |x, y| {
          auto.loc = IVec2::new(x, y);
        }, || {
          println!("bad loc: {val:?}");
        }),
        _ => println!("bad auto key: {key}")
      }
    });
    auto.initalize(&world.kinds);
    if tile_kind.0 != 0 {
      auto.tiles = vec![tile_kind; (auto.dim.x * auto.dim.y) as usize];
    }
    auto
  }

  pub fn ndx_to_loc(&self, ndx: usize) -> IVec2 {
    IVec2::new(ndx as i32 % self.dim.x, ndx as i32 / self.dim.x)
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AutoNdx(pub usize);
