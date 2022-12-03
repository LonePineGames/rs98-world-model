use crate::kind::{Kind, Kinds};

#[derive(Clone, Default)]
pub struct Pattern {
  pub only: Kind,
  pub input: Vec<Kind>,
  pub output: Vec<Kind>,
}

pub struct Patterns {
  pub patterns: Vec<Pattern>,
}

impl Patterns {
  pub fn new_test(kinds: &Kinds) -> Patterns {
    Patterns {
      patterns: vec![
        Pattern {
          only: kinds.get("machine"),
          input: vec![kinds.get("rock")],
          output: vec![kinds.get("thing")],
        },
      ],
    }
  }

  pub fn get(&self, kind: Kind, holding: &Vec<Kind>) -> Option<Pattern> {
    for pattern in &self.patterns {
      if pattern.only == kind {
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
}
