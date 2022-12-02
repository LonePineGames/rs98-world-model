#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Kind(pub usize);

pub struct KindData {
  pub name: String,
}

pub struct Kinds {
  pub kinds: Vec<KindData>,
}

impl Kinds {
  pub fn new_test() -> Kinds {
    let mut kinds: Vec<KindData> = vec![];
    kinds.push(KindData { name: "nothing".to_string() });
    kinds.push(KindData { name: "missingno".to_string() });
    kinds.push(KindData { name: "grass".to_string() });
    kinds.push(KindData { name: "rock".to_string() });
    kinds.push(KindData { name: "robo".to_string() });
    kinds.push(KindData { name: "machine".to_string() });
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
}
