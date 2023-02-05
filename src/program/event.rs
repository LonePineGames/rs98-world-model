use std::collections::HashMap;

use bevy::prelude::IVec2;
use conniver::Val;

use crate::model::{world::World, auto::AutoNdx, act::Action, dir::Dir};

use super::program::ProgramSpace;

pub type EventHandler = fn(Vec<Val>, &mut ProgramSpace, &mut World, AutoNdx);

pub fn get_event_handlers() -> HashMap<String, EventHandler> {
  let mut handlers = HashMap::<String, EventHandler>::new();
  handlers.insert("goto".to_string(), ev_goto);
  handlers.insert("move".to_string(), ev_move);
  handlers.insert("pick".to_string(), ev_pick);
  handlers.insert("place".to_string(), ev_place);
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

pub fn ev_move(args: Vec<Val>, program: &mut ProgramSpace, world: &mut World, ndx: AutoNdx) {
  println!("ev_move: {:?}", args);
  if args.len() < 2 {
    return;
  }
  let d = if let Val::Sym(d) = &args[1] {
    d
  } else {
    return;
  };
  let d = Dir::from_str(d);

  world.set_auto_action(ndx, Action::Move(d));
}

pub fn ev_pick(args: Vec<Val>, program: &mut ProgramSpace, world: &mut World, ndx: AutoNdx) {
  println!("ev_pick: {:?}", args);

  world.set_auto_action(ndx, Action::Pick(world.kinds.nothing(), world.kinds.nothing()));
}

pub fn ev_place(args: Vec<Val>, program: &mut ProgramSpace, world: &mut World, ndx: AutoNdx) {
  println!("ev_place: {:?}", args);

  world.set_auto_action(ndx, Action::Place(world.kinds.nothing()));
}
