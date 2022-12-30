use std::collections::HashMap;

use bevy::prelude::IVec2;
use conniver::Val;

use crate::model::{world::World, auto::AutoNdx, act::Action};

use super::program::ProgramSpace;

pub type EventHandler = fn(Vec<Val>, &mut ProgramSpace, &mut World, AutoNdx);

pub fn get_event_handlers() -> HashMap<String, EventHandler> {
  let mut handlers = HashMap::<String, EventHandler>::new();
  handlers.insert("goto".to_string(), ev_goto);
  handlers
}

pub fn ev_goto(args: Vec<Val>, program: &mut ProgramSpace, world: &mut World, ndx: AutoNdx) {
  println!("ev_goto: {:?}", args);
  if args.len() < 3 {
    return;
  }
  let x = if let Val::Num(x) = args[1] {
    x as i32
  } else {
    return;
  };
  let y = if let Val::Num(y) = args[2] {
    y as i32
  } else {
    return;
  };

  world.set_auto_action(ndx, Action::Goto(IVec2::new(x, y)));
}
