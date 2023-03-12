
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitField(u64);

impl BitField {
  pub fn new() -> Self {
    BitField(0)
  }

  pub fn get(&self, ndx: BFNDX) -> bool {
    (self.0 & ndx.0) != 0
  }

  pub fn set(&mut self, ndx: BFNDX, val: bool) {
    if val {
      self.0 |= ndx.0;
    } else {
      self.0 &= !ndx.0;
    }
  }

  fn toggle(&mut self, exists: BFNDX) {
    self.0 ^= exists.0;
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BFNDX(u64);

impl BFNDX {
  pub const fn new(ndx: u64) -> Self {
    BFNDX(1 << ndx)
  }
}

#[test]
fn test_bitfield() {
  let mut bf = BitField::new();
  let exists = BFNDX::new(0);
  let alive = BFNDX::new(1);
  let idle = BFNDX::new(2);

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