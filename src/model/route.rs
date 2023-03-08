use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

use bevy::prelude::IVec2;

use crate::model::{world::World, dir::Dir, auto::AutoNdx};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct RouteNode {
  cost: usize,
  pos: IVec2,
}

impl Ord for RouteNode {
  fn cmp(&self, other: &Self) -> Ordering {
      // Notice that the we flip the ordering on costs.
      // In case of a tie we compare positions - this step is necessary
      // to make implementations of `PartialEq` and `Ord` consistent.
      other.cost.cmp(&self.cost)
          .then_with(|| self.pos.x.cmp(&other.pos.x))
          .then_with(|| self.pos.y.cmp(&other.pos.y))
  }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for RouteNode {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

pub fn route(world: &World, auto: AutoNdx, dest: IVec2) -> Dir {
  let auto = world.get_auto(auto);
  let start = auto.loc;
  let kind = auto.kind;
  let parent = auto.parent;

  let mut heap = BinaryHeap::new();
  let mut dist = HashMap::<IVec2, usize>::new();
  let mut prev = HashMap::<IVec2, Dir>::new();
  let mut next = HashMap::<IVec2, Dir>::new();

  dist.insert(dest, 0);
  heap.push(RouteNode {
    pos: dest,
    cost: 0,
  });

  while let Some(RouteNode { cost, pos }) = heap.pop() {
    // Alternatively we could have continued to find all shortest paths
    if pos == start { break; }

    // Important as we may have already found a better way
    let prev_cost = dist.get(&pos).copied().unwrap_or(usize::MAX);
    if cost > prev_cost { continue; }

    // For each node we can reach, see if we can find a way with
    // a lower cost going through this node
    for dir in Dir::all() {
      let next_pos = pos + dir.to_ivec2();
      if !world.traction_valid(parent, kind, next_pos) { continue; }
      let to = RouteNode { cost: cost + 1, pos: next_pos };

      // If so, add it to the frontier and continue
      let prev_cost = dist.get(&to.pos).copied().unwrap_or(usize::MAX);
      if to.cost < prev_cost {
          heap.push(to);
          // Relaxation, we have now found a better way
          dist.insert(to.pos, to.cost);
          prev.insert(to.pos, dir);
          next.insert(pos, dir);
      }
    }
  }

  let dir = prev.get(&start).copied();
  if let Some(dir) = dir {
    dir.invert()
  } else {
    Dir::None
  }
}
