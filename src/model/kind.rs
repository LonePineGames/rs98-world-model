use bevy::prelude::IVec2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Kind(pub usize);

pub struct KindData {
  pub name: String,
  pub scene: String,
  pub item_dim: IVec2,
  pub traction: i32,
}

pub struct Kinds {
  pub kinds: Vec<KindData>,
}

impl Kinds {
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
    Kinds { kinds }
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
}
