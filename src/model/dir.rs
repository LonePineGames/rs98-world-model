use bevy::prelude::IVec2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Dir {
  #[default]
  North,
  East,
  South,
  West,
  None,
}

impl Dir {
  pub(crate) fn to_ivec2(self) -> IVec2 {
    match self {
      Dir::North => IVec2::new(0, 1),
      Dir::East => IVec2::new(1, 0),
      Dir::South => IVec2::new(0, -1),
      Dir::West => IVec2::new(-1, 0),
      Dir::None => IVec2::new(0, 0),
    }
  }

  // pub fn from_ivec2(auto_loc: IVec2) -> Dir {
  //   if auto_loc.x > 0 {
  //     Dir::East
  //   } else if auto_loc.x < 0 {
  //     Dir::West
  //   } else if auto_loc.y > 0 {
  //     Dir::North
  //   } else if auto_loc.y < 0 {
  //     Dir::South
  //   } else {
  //     Dir::None
  //   }
  // }

  pub fn from_str(c: &str) -> Dir {
    match c {
      "n" => Dir::North,
      "e" => Dir::East,
      "s" => Dir::South,
      "w" => Dir::West,
      _ => Dir::None,
    }
  }

  pub fn all() -> Vec<Dir> {
    vec![Dir::North, Dir::East, Dir::South, Dir::West]
  }

  pub fn invert(&self) -> Dir {
    match self {
      Dir::North => Dir::South,
      Dir::East => Dir::West,
      Dir::South => Dir::North,
      Dir::West => Dir::East,
      Dir::None => Dir::None,
    }
  }
}
