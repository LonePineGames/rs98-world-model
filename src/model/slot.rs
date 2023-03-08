use bevy::prelude::IVec2;

use super::auto::AutoNdx;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Slot(pub AutoNdx, pub IVec2);
