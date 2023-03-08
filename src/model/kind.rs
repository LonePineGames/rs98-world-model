use std::collections::HashMap;

use bevy::prelude::IVec2;
use conniver::{Val, read_object, read_ivec2, object::read_string, p};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Kind(pub usize);
impl Kind {
  pub fn matches(&self, other: Kind) -> bool {
    self.0 == other.0 || other.0 == 1 || self.0 == 1
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum KindRole {
  Tile,
  #[default]
  Item,
  Auto,
}

#[derive(Clone, Debug, Default)]
pub struct KindData {
  pub name: String,
  pub scene: String,
  pub item_dim: IVec2,
  pub program: Val,
  pub traction: i32,
  pub role: KindRole,
}

pub struct Kinds {
  pub kinds: Vec<KindData>,
  pub kinds_by_name: HashMap<String, Kind>,
}

impl Kinds {
  pub fn new_blank() -> Kinds {
    let mut kinds = Kinds { kinds: vec![], kinds_by_name: HashMap::new() };
    kinds.set_by_val("nothing", p("(
      (traction 10)
    )"));
    kinds.set_by_val("missingno", p("(
      (traction 1)
    )"));
    kinds.set_by_val("space", p("(
      (traction 1)
    )"));
    kinds
  }

  #[cfg(test)]
  pub fn new_test() -> Kinds {
    let mut kinds = Kinds::new_blank();
    kinds.set_by_val("earth", p("(
      (dim 20 20)
      (traction 1)
    )"));
    kinds.set_by_val("grass", p("(
      (scene \"model/lab-tile.glb#Scene0\")
      (traction 1)
    )"));
    kinds.set_by_val("rock", p("(
      (scene \"model/baux.glb#Scene0\")
      (traction 1)
    )"));
    kinds.set_by_val("robo", p("(
      (scene \"model/r1000.glb#Scene0\")
      (dim (1 1))
      (traction 2)
    )"));
    kinds.set_by_val("machine", p("(
      (dim (2 1))
      (traction 1)
    )"));
    kinds.set_by_val("wall", p("(
      (scene \"model/lab-wall.glb#Scene0\")
      (traction 5)
    )"));
    kinds.set_by_val("thing", p("(
      (traction 5)
    )"));
    kinds.set_by_val("table", p("(
      (scene \"model/table.glb#Scene0\")
      (dim (2 1))
      (traction 5)
    )"));
    kinds.set_by_val("widget", p("(
      (scene \"model/widget.glb#Scene0\")
      (dim (1 1))
      (traction 5)
    )"));

    kinds
  }

  pub fn get(&self, arg: &str) -> Kind {
    match arg {
      "ground" => self.nothing(),
      "any" => self.missingno(),
      _ => {
        if let Some(kind) = self.kinds_by_name.get(arg) {
          *kind
        } else {
          self.missingno()
        }
      }
    }
  }

  pub fn nothing(&self) -> Kind {
    Kind(0)
  }

  pub fn missingno(&self) -> Kind {
    Kind(1)
  }

  pub fn get_data(&self, kind: Kind) -> &KindData {
    &self.kinds[kind.0]
  }

  fn get_data_mut(&mut self, kind: Kind) -> &mut KindData {
    &mut self.kinds[kind.0]
  }

  pub fn set_by_val(&mut self, name: &str, data: Val) {
    let mut kind_data = KindData::default();
    kind_data.name = name.to_string();

    if kind_data.name == "" {
      println!("bad kind: {:?}", data);
      return;
    }

    // check if we already have this kind
    let kind = if let Some(kind) = self.kinds_by_name.get(&kind_data.name) {
      *kind
    } else {
      let kind = Kind(self.kinds.len());
      self.kinds_by_name.insert(kind_data.name.clone(), kind);
      self.kinds.push(kind_data);
      kind
    };
    let kind_data = self.get_data_mut(kind);
    let mut new_name = None;

    // now read the data
    read_object(&data, |key, val| {
      match key {
        "name" => new_name = Some(read_string(val)), // allows renaming
        "role" => kind_data.role = match read_string(val).as_str() {
          "tile" => KindRole::Tile,
          "item" => KindRole::Item,
          "auto" => KindRole::Auto,
          _ => {
            println!("bad role: {:?}", val);
            KindRole::Item
          }
        },
        "scene" => kind_data.scene = read_string(val),

        "dim" => read_ivec2(val, |x, y| {
          kind_data.item_dim = IVec2::new(x, y);
        }, || {
          println!("bad item-dim: {:?}", val);
        }),

        "traction" => if let Val::Num(i) = val {
          kind_data.traction = *i as i32;
        } else {
          println!("bad traction: {:?}", val);
        },

        "program" => kind_data.program = val.clone(),

        _ => {}
      }
    });

    // if we renamed, update the name map
    if let Some(new_name) = new_name {
      let old_name = kind_data.name.clone();
      kind_data.name = new_name.clone();
      self.kinds_by_name.remove(&old_name);
      self.kinds_by_name.insert(new_name, kind);
    }
  }

  pub fn name(&self, kind: Kind) -> String {
    self.kinds[kind.0].name.clone()
  }

  pub fn action_name(&self, kind: Kind) -> String { 
    match kind {
      Kind(0) => "ground".to_string(),
      Kind(1) => "any".to_string(),
      _ => self.name(kind)
    }
  }

  #[cfg(test)]
  pub fn name_list(&self, kinds: &[Kind]) -> String {
    let mut names = vec![];
    for kind in kinds {
      names.push(self.name(*kind));
    }
    names.join(" ")
  }

  #[cfg(test)]
  pub fn action_name_list(&self, kinds: &[Kind]) -> String {
    let mut names = vec![];
    for kind in kinds {
      names.push(self.action_name(*kind));
    }
    names.join(" ")
  }
}
