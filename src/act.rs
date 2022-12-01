use bevy::prelude::IVec2;

use crate::{kind::Kind, world::World, auto::AutoNdx, dir::Dir};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Action {
  #[default]
  Stop,
  Move(Dir),
  Pick(Kind, Kind),
  Place(Kind),
}

impl Action {
  pub fn act(&self, world: &mut World, auto: AutoNdx) -> Option<String> {
    match self {
      Action::Stop => {
        None
      }

      Action::Move(dir) => {
        let auto = world.get_auto_mut(auto);
        let new_loc = auto.loc + dir.to_ivec2();
        auto.loc = new_loc;
        None
      }

      Action::Pick(item, source) => {
        let auto_data = world.get_auto(auto);
        let loc = auto_data.loc;
        let parent = auto_data.parent;
        let pick_up = world.get_item(parent, loc);
        if source != &world.kinds.nothing() {
          Some(format!("Could not find {:?} in {:?}.", item, source))
        } else if pick_up != *item {
          Some(format!("Could not find {:?} in {:?}.", item, source))
        } else {
          world.set_item(parent, loc, world.kinds.nothing());
          world.set_item(auto, IVec2::new(0, 0), *item);
          None
        }
      }

      Action::Place(item) => {
        let auto_data = world.get_auto(auto);
        let holding_kind = world.get_item(auto, IVec2::new(0, 0));
        let loc = auto_data.loc;
        let parent = auto_data.parent;
        let place = world.get_item(parent, loc);
        if place != world.kinds.nothing() {
          Some(format!("Could not place {:?} in {:?}.", item, place))
        } else {
          world.set_item(parent, loc, holding_kind);
          world.set_item(auto, IVec2::new(0, 0), world.kinds.nothing());
          None
        }
      }
    }
  }
}
