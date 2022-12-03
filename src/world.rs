
use bevy::prelude::IVec2;

use crate::{auto::{Auto, AutoNdx}, kind::{Kind, Kinds}, act::Action, dir::Dir, pattern::{Pattern, Patterns}};

pub struct World {
  pub autos: Vec<Auto>,
  pub kinds: Kinds,
  pub patterns: Patterns,
}

impl World {
  pub fn new_test() -> World {
    let kinds = Kinds::new_test();
    let mut world = World {
      patterns: Patterns::new_test(&kinds),
      kinds,
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
    let new = new.initalize();
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
    println!("Action time: {}", auto.action_time);
    if auto.action_time >= 1.0 {
      auto.action_time = 0.0;
      let action = auto.action;
      let stall_message = action.act(self, ndx);
      let auto = self.get_auto_mut(ndx);
      auto.stall_message = stall_message;
    }
  }

  pub fn finish_auto_action(&mut self, ndx: AutoNdx) {
    let auto = self.get_auto_mut(ndx);
    auto.action = Action::Stop;
    auto.action_time = 0.0;
  }

  pub fn set_tile(&mut self, space: AutoNdx, loc: IVec2, kind: Kind) {
    let space = self.get_auto_mut(space);
    let ndx = space.get_ndx(loc);
    if ndx >= 0 && ndx < space.tiles.len() as i32 {
      space.tiles[ndx as usize] = kind;
    }
  }

  pub fn get_autos_at(&self, parent_ndx: AutoNdx, loc: IVec2) -> Vec<AutoNdx> {
    let mut ndxes = vec![];
    for (ndx, auto) in self.autos.iter().enumerate() {
      if auto.parent == parent_ndx && auto.loc == loc {
        ndxes.push(AutoNdx(ndx));
      }
    }
    ndxes
  }

  pub fn traction_valid(&self, parent: AutoNdx, auto: Kind, pos: IVec2) -> bool {
    let auto = self.kinds.get_data(auto);
    let ground = self.get_tile(parent, pos);
    let ground = self.kinds.get_data(ground);
    return auto.traction > ground.traction;
  }

  fn get_tile(&self, parent: AutoNdx, pos: IVec2) -> Kind {
    let parent = self.get_auto(parent);
    let ndx = parent.get_ndx(pos);
    if ndx >= 0 && ndx < parent.tiles.len() as i32 {
      parent.tiles[ndx as usize]
    } else {
      self.kinds.nothing()
    }
  }

  pub fn set_all_tiles(&mut self, space_ndx: AutoNdx, tile_kind: Kind) {
    let space = self.get_auto(space_ndx);
    let dim = space.dim;
    for y in 0..dim.y {
      for x in 0..dim.x {
        let loc = IVec2::new(x, y);
        self.set_tile(space_ndx, loc, tile_kind);
      }
    }
  }

  pub fn get_items(&self, auto: AutoNdx) -> Vec<Kind> {
    let auto = self.get_auto(auto);
    auto.items.clone()
  }

  pub fn get_pattern(&self, kind: Kind, holding: &Vec<Kind>) -> Option<Pattern> {
    self.patterns.get(kind, holding)
  }
}