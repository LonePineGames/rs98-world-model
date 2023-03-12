
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitField(u64);

impl BitField {
  pub fn get(&self, ndx: BFNdx) -> bool {
    (self.0 & ndx.0) != 0
  }

  pub fn set(&mut self, ndx: BFNdx, val: bool) {
    if val {
      self.0 |= ndx.0;
    } else {
      self.0 &= !ndx.0;
    }
  }

  #[cfg(test)]
  fn toggle(&mut self, exists: BFNdx) {
    self.0 ^= exists.0;
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BFNdx(u64);

impl BFNdx {
  pub const fn new(ndx: u64) -> Self {
    BFNdx(1 << ndx)
  }
}

#[test]
fn test_bitfield() {
  let mut bf = BitField::default();
  let exists = BFNdx::new(0);
  let alive = BFNdx::new(1);
  let idle = BFNdx::new(2);

  assert_eq!(bf.get(exists), false);
  assert_eq!(bf.get(alive), false);
  assert_eq!(bf.get(idle), false);

  bf.set(exists, true);
  bf.set(alive, true);
  bf.set(idle, false);

  assert_eq!(bf.get(exists), true);
  assert_eq!(bf.get(alive), true);
  assert_eq!(bf.get(idle), false);

  bf.set(exists, false);

  assert_eq!(bf.get(exists), false);
  assert_eq!(bf.get(alive), true);
  assert_eq!(bf.get(idle), false);

  bf.toggle(exists);
  assert_eq!(bf.get(exists), true);
}