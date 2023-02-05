use std::collections::HashMap;

use bevy::prelude::IVec2;
use conniver::Val;

use crate::model::{auto::AutoNdx, world::World};

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
    println!("print: {:?}", args[1..].iter().map(|v| v.to_string()).collect::<Vec<String>>().join(""));
    None
  });

  handlers
}
