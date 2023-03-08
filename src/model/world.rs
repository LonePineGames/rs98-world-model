
use bevy::{prelude::{IVec2, Resource, Plugin, App, ResMut, Res}, time::Time};
use conniver::Val;

use crate::model::{auto::{Auto, AutoNdx}, kind::{Kind, Kinds}, act::Action, pattern::{Pattern, Patterns}, slot::Slot};

#[derive(Resource)]
pub struct World {
  pub autos: Vec<Auto>,
  pub kinds: Kinds,
  pub patterns: Patterns,
}

impl World {
  pub fn new_blank() -> World {
    let kinds = Kinds::new_blank();
    let mut world = World {
      patterns: Patterns::new_blank(),
      kinds,
      autos: vec![],
    };
    world.create_auto(Auto {
      kind: world.kinds.get("space"),
      dim: IVec2::new(100, 100),
      ..Auto::default()
    });
    world
  }

  #[cfg(test)]
  pub fn new_test() -> World {
    let kinds = Kinds::new_test();
    let mut world = World {
      patterns: Patterns::new_test(&kinds),
      kinds,
      autos: vec![],
    };
    world.create_auto(Auto {
      kind: world.kinds.get("space"),
      dim: IVec2::new(100, 100),
      ..Auto::default()
    });
    world
  }

  /*#[cfg(test)]
  pub fn new_lab() -> World {
    let kinds = Kinds::new_test();
    let dim = IVec2::new(50, 50);
    let mut world = World {
      patterns: Patterns::new_test(&kinds),
      kinds,
      autos: vec![],
    };
    world.create_auto(Auto {
      kind: world.kinds.get("earth"),
      dim,
      ..Auto::default()
    });
    let earth = AutoNdx(0);

    for x in 0..dim.x {
      for y in 0..dim.y {
        let loc = IVec2::new(x, y);
        let edge = x == 0 || x == dim.x - 1 || y == 0 || y == dim.y - 1;
        let internal_wall_x = x % 20 == 0;
        let internal_wall_y = y % 10 == 9;
        let door = y % 20 < 2 || y % 20 > 17 || (x > 20 && x < 30);
        let internal_wall = (internal_wall_x || internal_wall_y) && !door;
        let item = if edge || internal_wall {
          world.kinds.get("wall")
        } else {
          world.kinds.get("grass")
        };
        world.set_tile(earth, loc, item);
      }
    }

    world.create_auto(Auto {
      kind: world.kinds.get("robo"),
      parent: earth,
      loc: IVec2::new(2, 1),
      ..Auto::default()
    });

    world.create_auto(Auto {
      kind: world.kinds.get("table"),
      parent: earth,
      loc: IVec2::new(2, 6),
      ..Auto::default()
    });

    world.set_item(earth, IVec2::new(3, 2), world.kinds.get("rock"));

    world
  }*/

  pub fn create_auto(&mut self, new: Auto) -> AutoNdx {
    let new = new.initalize(&self.kinds);
    let result = AutoNdx(self.autos.len());
    if new.parent != result {
      self.get_auto_mut(new.parent).children.push(result);
    }
    self.autos.push(new);
    result
  }

  pub fn create_auto_from_val(&mut self, val: Val) -> AutoNdx {
    let new = Auto::from_val(val, &self.kinds);
    self.create_auto(new)
  }

  pub fn get_auto_mut(&mut self, auto_ndx: AutoNdx) -> &mut Auto {
    &mut self.autos[auto_ndx.0]
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
    auto.action_finished = false;
  }

  pub fn get_auto_action(&self, auto: AutoNdx) -> Action {
    let auto = self.get_auto(auto);
    auto.action
  }

  #[cfg(test)]
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
    if !auto.action_finished {
      auto.action_time += dur;
      if auto.action_time >= 1.0 {
        auto.action_time = 0.0;
        let action = auto.action;
        let stall_message = action.act(self, ndx);
        let auto = self.get_auto_mut(ndx);
        auto.stall_message = stall_message;
        let auto = self.get_auto(ndx);
        if let Some(message) = &auto.stall_message {
          println!("{}: {}", self.kinds.get_data(auto.kind).name, message);
        }
      }
    }
  }

  pub fn finish_auto_action(&mut self, ndx: AutoNdx) {
    let auto = self.get_auto_mut(ndx);
    auto.action_finished = true;
    //auto.action = Action::Stop;
    auto.action_time = 0.0;
  }

  #[cfg(test)]
  pub fn set_tile(&mut self, space: AutoNdx, loc: IVec2, kind: Kind) {
    let space = self.get_auto_mut(space);
    let ndx = space.get_ndx(loc);
    if ndx >= 0 && ndx < space.tiles.len() as i32 {
      space.tiles[ndx as usize] = kind;
    }
  }

  pub fn get_slots(&self, parent_ndx: AutoNdx, loc: IVec2) -> Vec<Slot> {
    let mut ndxes = vec![];
    let children = self.get_auto(parent_ndx).children.clone();
    for ndx in children {
      let auto = self.get_auto(ndx);
      if auto.parent != parent_ndx {
        println!("{} != {}", auto.parent.0, parent_ndx.0);
        continue;
      }
      let dim = auto.dim;
      let kind_name = self.kinds.get_data(auto.kind).name.clone();
      println!("{} {:?}: {:?} {:?} {:?}", kind_name, ndx, loc, auto.loc, dim);
      let loc = loc - auto.loc;
      if loc.x >= 0 && loc.x < dim.x && loc.y >= 0 && loc.y < dim.y {
        ndxes.push(Slot(ndx, loc));
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

  pub fn get_tile(&self, parent: AutoNdx, pos: IVec2) -> Kind {
    let parent = self.get_auto(parent);
    let ndx = parent.get_ndx(pos);
    if ndx >= 0 && ndx < parent.tiles.len() as i32 {
      parent.tiles[ndx as usize]
    } else {
      self.kinds.nothing()
    }
  }

  #[cfg(test)]
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

  pub fn pick_place_target(&self, auto_ndx: AutoNdx, target_kind: Kind, item_kind: Kind) -> Option<Slot> {
    let auto = self.get_auto(auto_ndx);
    let parent_ndx = auto.parent;
    let loc = auto.loc;
    let mut result = None;
    if target_kind == self.kinds.nothing() {
      result = Some(Slot(parent_ndx, loc));
    } else {
      let slots = self.get_slots(parent_ndx, loc);
      println!("slots: {:?} {:?} {:?}", slots, auto_ndx, parent_ndx);
      for slot in slots {
        if slot.0 == auto_ndx {
          continue;
        }
        let auto = self.get_auto(slot.0);
        println!("slot: {:?} {:?} {:?}", slot, auto.kind, target_kind);
        if auto.kind.matches(target_kind) {
          let item_there = self.get_item(slot.0, slot.1);
          println!("item_there: {:?} {:?} {:?}", item_there, item_there, item_kind);
          if item_there.matches(item_kind) && (item_there != Kind(0) || item_kind == Kind(0)) {
            result = Some(slot);
            break;
          }
        }
      }

      if target_kind == Kind(1) && result.is_none() {
        result = Some(Slot(parent_ndx, loc));
      }
    }

    if let Some(result_slot) = result {
      let item_there = self.get_item(result_slot.0, result_slot.1);
      if !item_there.matches(item_kind) || (item_there == Kind(0) && item_kind != Kind(0)) {
        result = None;
      }
    }
    
    result
  }
}

pub struct RS98WorldPlugin;

impl Plugin for RS98WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(World::new_blank())
      .add_system(update_world);
  }
}

pub fn update_world(mut world: ResMut<World>, time: Res<Time>) {
  world.update(time.delta_seconds_f64()*4.0);
}
