use bevy::prelude::IVec2;

use crate::model::{kind::Kind, world::World, auto::AutoNdx, dir::Dir, route::route, slot::Slot};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Action {
  #[default]
  Stop,
  Move(Dir),
  Pick(Kind, Kind),
  Place(Kind),
  Goto(IVec2),
  Produce,
  Fire(AutoNdx),
}

impl Action {
  pub fn act(&self, world: &mut World, auto_ndx: AutoNdx) -> Option<String> {
    let loc = world.get_auto(auto_ndx).loc;
    println!("Action: {:?} {}x{}", self, loc.x, loc.y);
    match self {
      Action::Stop => {
        world.finish_auto_action(auto_ndx);
        None
      }

      Action::Move(dir) => {
        let auto = world.get_auto(auto_ndx);
        let new_loc = auto.loc + dir.to_ivec2();
        let parent = auto.parent;
        let kind = auto.kind;
        if world.traction_valid(parent, kind, new_loc) {
          let auto = world.get_auto_mut(auto_ndx);
          auto.loc = new_loc;
          world.finish_auto_action(auto_ndx);
          None
        } else {
          let tile = world.get_tile(parent, new_loc);
          let tile_name = &world.kinds.get_data(tile).name;
          let auto_name = &world.kinds.get_data(kind).name;
          Some(format!("Could not move to ({},{}): {} cannot cross {}.", new_loc.x, new_loc.y, auto_name, tile_name))
        }
      }

      Action::Pick(item, source) => {
        let auto_data = world.get_auto(auto_ndx);
        let loc = auto_data.loc;
        let parent = auto_data.parent;
        let item_kind_name = &world.kinds.get_data(*item).name.clone();
        let source_kind_name = &world.kinds.get_data(*source).name.clone();

        if source == &world.kinds.nothing() {
          if world.has_item(parent, loc) {
            let item = world.get_item(parent, loc);
            world.set_item(parent, loc, world.kinds.nothing());
            world.set_item(auto_ndx, IVec2::new(0, 0), item);
            world.finish_auto_action(auto_ndx);
            println!("Picked up {} from ground.", item_kind_name);
            None
          } else {
            Some(format!("Could not find {} on ground.", item_kind_name))
          }
        } else {
          for Slot(other_ndx, pick_loc) in world.get_slots(parent, loc) {
            let other = world.get_auto(other_ndx);
            if other.kind == *source {
              let holding = world.get_item(other_ndx, pick_loc);
              if holding == *item {
                world.set_item(other_ndx, pick_loc, world.kinds.nothing());
                world.set_item(auto_ndx, pick_loc, holding);
                world.finish_auto_action(auto_ndx);
                return None;
              } else {
                return Some(format!("Could not find {} in {}.", item_kind_name, source_kind_name));
              }
            }
          }
          Some(format!("Could not find {} at {}.", source_kind_name, loc))
          
        }

        /*if source != &world.kinds.nothing() {
          Some(format!("Could not find {:?} in {:?}.", item, source))
        } else if pick_up != *item {
          Some(format!("Could not find {:?} in {:?}.", item, source))
        } else {
          world.set_item(parent, loc, world.kinds.nothing());
          world.set_item(auto, IVec2::new(0, 0), *item);
          world.finish_auto_action(auto);
          None
        }*/
      }

      Action::Place(dest) => {
        let auto_data = world.get_auto(auto_ndx);
        let holding_kind = world.get_item(auto_ndx, IVec2::new(0, 0));
        let loc = auto_data.loc;
        let parent = auto_data.parent;
        let mut target = None;
        if dest == &world.kinds.nothing() {
          target = Some(Slot(parent, loc));
        } else {
          for Slot(other_ndx, place_loc) in world.get_slots(parent, loc) {
            let other = world.get_auto(other_ndx);
            if other.kind == *dest {
              target = Some(Slot(other_ndx, place_loc));
              break;
            }
          }
        }

        if let Some(Slot(target_ndx, target_loc)) = target {
          if world.has_item(target_ndx, target_loc) {
            return Some("Location is not empty.".to_owned())
          } else {
            world.set_item(target_ndx, target_loc, holding_kind);
            world.set_item(auto_ndx, IVec2::new(0, 0), world.kinds.nothing());
            world.finish_auto_action(auto_ndx);
            return None;
          }
        } else {
          let dest_name = &world.kinds.get_data(*dest).name;
          return Some(format!("Could not find {}.", dest_name))
        }
      }

      Action::Goto(loc) => {
        let dir = route(world, auto_ndx, *loc);
        //let dir = Dir::from_ivec2(*loc - auto_loc);
        println!("dir: {:?}", dir);
        if dir == Dir::None {
          world.finish_auto_action(auto_ndx);
          None
        } else {
          let auto = world.get_auto_mut(auto_ndx);
          let new_loc = auto.loc + dir.to_ivec2();
          auto.loc = new_loc;
          None
        }
      }

      Action::Produce => {
        let auto_data = world.get_auto(auto_ndx);
        let holding = world.get_items(auto_ndx);
        println!("holding: {:?}", holding);
        let pattern = world.get_pattern(auto_data.kind, &holding);
        if let Some(pattern) = pattern {
          for (ndx, kind) in pattern.output.iter().enumerate() {
            let loc = world.get_auto(auto_ndx).ndx_to_loc(ndx);
            world.set_item(auto_ndx, loc, *kind);
          }
          world.finish_auto_action(auto_ndx);
          None
        } else {
          Some(format!("Could not find pattern for {:?} with {:?}", auto_data.kind, holding))
        }
      }

      Action::Fire(other) => {
        let my_loc = world.get_auto(auto_ndx).loc;
        let other_loc = world.get_auto(*other).loc;
        let dist = my_loc - other_loc;
        let dist = dist.x.abs() + dist.y.abs();
        if dist <= 5 {
          world.get_auto_mut(*other).alive = false;
          world.finish_auto_action(auto_ndx);
          None
        } else {
          Some("Target out of range.".to_string())
        }
      }
    }
  }
}
