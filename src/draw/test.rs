use crate::model::{world::World, auto::Auto};

use super::entities::Entities;

#[test]
fn test_entities() {
  let mut world = World::new_test();
  let mut entities = Entities::new(world);

  assert_eq!(entities.count(), 20);
}