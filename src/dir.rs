use bevy::prelude::IVec2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Dir {
  #[default]
  North,
  East,
  South,
  West,
}

impl Dir {
  pub(crate) fn to_ivec2(&self) -> IVec2 {
    match self {
      Dir::North => IVec2::new(0, 1),
      Dir::East => IVec2::new(1, 0),
      Dir::South => IVec2::new(0, -1),
      Dir::West => IVec2::new(-1, 0),
    }
  }
}
