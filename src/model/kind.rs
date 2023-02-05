use std::collections::HashMap;

use bevy::prelude::IVec2;
use conniver::{Val, read_object, read_ivec2, object::read_string};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Kind(pub usize);

#[derive(Clone, Debug, Default)]
pub struct KindData {
  pub name: String,
  pub scene: String,
  pub item_dim: IVec2,
  pub traction: i32,
}

pub struct Kinds {
  pub kinds: Vec<KindData>,
  pub kinds_by_name: HashMap<String, Kind>,
}

impl Kinds {
  pub fn new_blank() -> Kinds {
    Kinds { kinds: vec![], kinds_by_name: HashMap::new() }
  }

  pub fn new_test() -> Kinds {
    let mut kinds: Vec<KindData> = vec![];
    kinds.push(KindData { 
      name: "nothing".to_string(),
      scene: "".to_string(),
      item_dim: IVec2::new(0, 0),
      traction: 10,
    });
    kinds.push(KindData { 
      name: "missingno".to_string(),
      scene: "".to_string(),
      item_dim: IVec2::new(0, 0),
      traction: 1,
    });
    kinds.push(KindData { 
      name: "grass".to_string(),
      scene: "model/lab-tile.glb#Scene0".to_string(),
      item_dim: IVec2::new(0, 0),
      traction: 1,
    });
    kinds.push(KindData { 
      name: "rock".to_string(),
      scene: "model/baux.glb#Scene0".to_string(),
      item_dim: IVec2::new(0, 0),
      traction: 1,
    });
    kinds.push(KindData { 
      name: "robo".to_string(),
      scene: "model/r1000.glb#Scene0".to_string(),
      item_dim: IVec2::new(1, 1),
      traction: 2,
    });
    kinds.push(KindData { 
      name: "machine".to_string(),
      scene: "".to_string(),
      item_dim: IVec2::new(2, 1),
      traction: 1,
    });
    kinds.push(KindData { 
      name: "wall".to_string(),
      scene: "model/lab-wall.glb#Scene0".to_string(),
      item_dim: IVec2::new(0, 0),
      traction: 5,
    });
    kinds.push(KindData { 
      name: "thing".to_string(),
      scene: "".to_string(),
      item_dim: IVec2::new(0, 0),
      traction: 5,
    });
    kinds.push(KindData {
      name: "table".to_string(),
      scene: "model/table.glb#Scene0".to_string(),
      item_dim: IVec2::new(2, 1),
      traction: 5,
    });
    let mut kinds_by_name = HashMap::new();
    for (i, kind) in kinds.iter().enumerate() {
      kinds_by_name.insert(kind.name.clone(), Kind(i));
    }
    Kinds { kinds, kinds_by_name }
  }

  pub fn get(&self, arg: &str) -> Kind {
    for (i, kind) in self.kinds.iter().enumerate() {
      if kind.name == arg {
        return Kind(i);
      }
    }
    return self.missingno();
  }

  pub fn nothing(&self) -> Kind {
    Kind(0)
  }

  pub fn missingno(&self) -> Kind {
    Kind(0)
  }

  pub fn get_data(&self, kind: Kind) -> &KindData {
    &self.kinds[kind.0]
  }

  fn get_data_mut(&mut self, kind: Kind) -> &mut KindData {
    &mut self.kinds[kind.0]
  }

  pub fn set_by_val(&mut self, data: Val) {
    let mut kind_data = KindData::default();

    // read once for the name
    read_object(&data, |key, val| {
      match key {
        "name" => kind_data.name = format!("{:?}", val),
        _ => {}
      }
    });

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

    // now read again for the rest
    read_object(&data, |key, val| {
      match key {
        "name" => kind_data.name = read_string(val),
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

        _ => {}
      }
    });
  }
}
