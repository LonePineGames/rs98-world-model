

use conniver::{Val, read_object};

use crate::model::kind::{Kind};

#[cfg(test)]
use crate::model::kind::Kinds;

use super::world::World;

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

  pub fn from_val(val: &Val, world: &World) -> Pattern {
    let mut pattern = Pattern::new();
    pattern.for_kind = Kind(1);
    let mut in_out = vec![vec![], vec![]];
    let mut bad = false;
    read_object(val, |key, val| {
      if key == "in" || key == "out" {
        let ndx = if key == "in" { 0 } else { 1 };
        if let Val::List(list) = val {
          in_out[ndx] = list.clone();
        } else {
          bad = true;
        }
      } else if key == "for" {
        if let Val::Sym(kind) = val {
          pattern.for_kind = world.kinds.get(kind);
        } else {
          println!("bad only: {val:?}");
          bad = true;
        }
      }
    });
    if bad {
      //return Some(Val::String("usage: (define-pattern (for ...) (in ...) (out ...) ...)".to_owned()));
    }

    let in_out = in_out.into_iter().map(|v| {
      v.into_iter().map(|v| {
        if let Val::Sym(kind) = v {
          world.kinds.get(&kind)
        } else {
          Kind(1)
        }
      }).collect()
    }).collect::<Vec<Vec<Kind>>>();

    pattern.input = in_out[0].clone();
    pattern.output = in_out[1].clone();
    pattern
  }
}

#[derive(Debug)]
pub struct Patterns {
  pub patterns: Vec<Pattern>,
}

impl Patterns {
  pub fn new_blank() -> Patterns {
    Patterns { patterns: vec![] }
  }

  #[cfg(test)]
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
      if pattern.for_kind == kind && pattern.input.len() == holding.len() {
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
    None
  }

  #[cfg(test)]
  pub fn len(&self) -> usize {
    self.patterns.len()
  }
}
