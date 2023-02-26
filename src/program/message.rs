use std::collections::HashMap;

use bevy::prelude::IVec2;
use conniver::{Val, object::read_string};

use crate::model::{auto::AutoNdx, world::World, act::Action, kind::Kind, dir::Dir};

use super::program::ProgramSpace;


pub type MessageHandler = fn(Vec<Val>, &mut ProgramSpace, &mut World, AutoNdx) -> Option<Val>;

pub fn get_message_handlers() -> HashMap<String, MessageHandler> {
  let mut handlers = HashMap::<String, MessageHandler>::new();

  handlers.insert("item-at".to_string(), |args, _, world, _| {
    println!("item-at: {:?}", args);
    if args.len() < 4 {
      return Some(Val::String("usage: (item-at auto x y)".to_owned()));
    }
    let auto = if let Val::Num(auto) = &args[1] {
      AutoNdx(*auto as usize)
    } else {
      return Some(Val::String("usage: (item-at auto x y)".to_owned()));
    };
    let x = if let Val::Num(x) = args[2] {
      x as i32
    } else {
      return Some(Val::String("usage: (item-at auto x y)".to_owned()));
    };
    let y = if let Val::Num(y) = args[3] {
      y as i32
    } else {
      return Some(Val::String("usage: (item-at auto x y)".to_owned()));
    };
    let pos = IVec2::new(x, y);
    let item = world.get_item(auto, pos);
    println!("item-at: {:?}", item);
    let item_name = world.kinds.get_data(item).name.clone();
    Some(Val::String(item_name))
  });

  handlers.insert("print".to_string(), |args, _, _, _| {
    println!("{}", args[1..].iter().map(|v| read_string(v)).collect::<Vec<String>>().join(""));
    Some(Val::nil())
  });

  handlers.insert("move".to_string(), |args, _, world, auto| {
    println!("move: {:?}", args);
    if args.len() < 2 {
      return Some(Val::String("usage: (move auto dir)".to_owned()));
    }
    let dir = if let Val::Sym(dir) = &args[1] {
      Dir::from_str(&dir)
    } else {
      return Some(Val::String("usage: (move auto dir)".to_owned()));
    };
    action_handler(world, auto, Action::Move(dir))
  });

  handlers.insert("goto".to_string(), |args, _, world, auto| {
    println!("goto: {:?}", args);
    if args.len() < 3 {
      return Some(Val::String("usage: (goto auto x y)".to_owned()));
    }
    let x = if let Val::Num(x) = args[1] {
      x as i32
    } else {
      return Some(Val::String("usage: (goto auto x y)".to_owned()));
    };
    let y = if let Val::Num(y) = args[2] {
      y as i32
    } else {
      return Some(Val::String("usage: (goto auto x y)".to_owned()));
    };
    let pos = IVec2::new(x, y);

    action_handler(world, auto, Action::Goto(pos))
  });

  handlers.insert("stop".to_string(), |_, _, world, auto| {
    action_handler(world, auto, Action::Stop)
  });

  handlers.insert("pick".to_string(), |_, _, world, auto| {
    action_handler(world, auto, Action::Pick(Kind(0), Kind(0)))
  });

  handlers.insert("place".to_string(), |_, _, world, auto| {
    action_handler(world, auto, Action::Place(Kind(0)))
  });

  handlers.insert("define-kind".to_string(), |args, _, world, _| {
    if args.len() < 1 {
      return Some(Val::String("usage: (define-kind (name x) ...)".to_owned()));
    }
    let args = Val::List(args[1..].to_vec());
    world.kinds.set_by_val(args);
    Some(Val::nil())
  });

  handlers.insert("create-auto".to_string(), |args, _, world, _| {
    if args.len() < 1 {
      return Some(Val::String("usage: (create-auto (name x) ...)".to_owned()));
    }
    let args = Val::List(args[1..].to_vec());
    let auto = world.create_auto_from_val(args);
    Some(Val::Num(auto.0 as f32))
  });

  handlers.insert("access".to_string(), |args, program, _, _| {
    if args.len() < 1 {
      return Some(Val::String("usage: (access auto)".to_owned()));
    }
    let auto = if let Val::Num(auto) = &args[1] {
      AutoNdx(*auto as usize)
    } else {
      return Some(Val::String("usage: (access auto)".to_owned()));
    };
    program.access = auto;
    Some(Val::nil())
  });

  handlers
}

fn action_handler(world: &mut World, auto: AutoNdx, generator: Action) -> Option<Val> {
  let action = world.get_auto_action(auto);
  if action != generator {
    world.set_auto_action(auto, generator);
  } else if world.get_auto(auto).action_finished {
    println!("action_handler: finished");
    world.set_auto_action(auto, Action::Stop);
    return Some(Val::nil());
  }
  None
}
