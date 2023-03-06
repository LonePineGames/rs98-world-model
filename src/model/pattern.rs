use crate::model::kind::{Kind, Kinds};

#[derive(Clone, Default, Debug)]
pub struct Pattern {
  pub for_kind: Kind,
  pub input: Vec<Kind>,
  pub output: Vec<Kind>,
}

impl Pattern {
  pub fn new() -> Pattern {
    Pattern {
      for_kind: Kind(1),
      input: vec![],
      output: vec![],
    }
  }
}

#[derive(Debug)]
pub struct Patterns {
  pub patterns: Vec<Pattern>,
}

impl Patterns {
  pub fn new_blank(kinds: &Kinds) -> Patterns {
    Patterns { patterns: vec![] }
  }

  pub fn new_test(kinds: &Kinds) -> Patterns {
    Patterns {
      patterns: vec![
        Pattern {
          for_kind: kinds.get("machine"),
          input: vec![kinds.get("rock"), Kind(0)],
          output: vec![kinds.get("thing"), Kind(0)],
        },
        Pattern {
          for_kind: kinds.get("machine"),
          input: vec![kinds.get("thing"), kinds.get("rock")],
          output: vec![kinds.get("widget"), Kind(0)], 
        },
      ],
    }
  }

  pub fn add(&mut self, pattern: Pattern) {
    self.patterns.push(pattern);
  }

  pub fn get(&self, kind: Kind, holding: &Vec<Kind>) -> Option<Pattern> {
    for pattern in &self.patterns {
      if pattern.for_kind == kind {
        if pattern.input.len() == holding.len() {
          let mut found = true;
          for (i, input) in pattern.input.iter().enumerate() {
            if *input != holding[i] {
              found = false;
              break;
            }
          }
          if found {
            return Some(pattern.clone());
          }
        }
      }
    }
    None
  }

  pub fn len(&self) -> usize {
    self.patterns.len()
  }
}
