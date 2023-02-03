use std::collections::HashMap;

use bevy::prelude::*;
use conniver::{Val, State, eval_s, p};

use crate::model::{auto::AutoNdx, world::World};

use super::{event::{EventHandler, get_event_handlers}, message::{MessageHandler, get_message_handlers}};

pub struct RS98ProgramPlugin;

impl Plugin for RS98ProgramPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ProgramSpace::new(AutoNdx(1)))
      .add_system(update_program)
      .add_system(process_events)
      .add_system(process_messages)
      ;
  }
}

pub fn update_program(
  mut program: ResMut<ProgramSpace>,
  time: Res<Time>,
) {
  program.update(time.delta_seconds_f64());
}

pub fn process_events(
  mut program: ResMut<ProgramSpace>,
  mut world: ResMut<World>,
  time: Res<Time>,
) {
  program.process_events(&mut world);
}

pub fn process_messages(
  mut program: ResMut<ProgramSpace>,
  mut world: ResMut<World>,
  time: Res<Time>,
) {
  program.process_messages(&mut world);
}

#[derive(Resource)]
pub struct ProgramSpace {
  procs: Vec<State>,
  proto: State,
  pub access: AutoNdx,
  event_handlers: HashMap<String, EventHandler>,
  message_handlers: HashMap<String, MessageHandler>,
}

impl ProgramSpace {
  pub fn new(access: AutoNdx) -> Self {
    let mut proto = State::new();
    proto.load_lib();
    let message_handlers = get_message_handlers();
    for (name, _) in message_handlers.iter() {
      proto.message_add(name);
    }
    //eval_s(&p("(load \"assets/cnvr/lib.cnvr\")"), &mut proto);
    eval_s(&p("(load \"assets/cnvr/velocity.cnvr\")"), &mut proto);
    Self {
      procs: Vec::new(),
      proto,
      access,
      event_handlers: get_event_handlers(),
      message_handlers,
    }
  }

  pub fn ensure_size(&mut self, size: usize) {
    let size = size + 1;
    let old_size = self.procs.len();
    if size <= old_size {
      return;
    }
    self.procs.resize(size, self.proto.clone());
    for i in old_size..size {
      self.procs[i].set_var(&"me".to_string(), Val::Num(i as f32));
    }
  }

  pub fn set_program(&mut self, robo: AutoNdx, p: Val) {
    self.ensure_size(robo.0);
    self.procs[robo.0].set_program(p);
  }

  pub fn update(&mut self, dur: f64) {
    for state in &mut self.procs {
      if state.running() {
        println!("running");
        state.run();
      }
    }
  }

  pub fn process_events(&mut self, world: &mut World) {
    let mut events = vec![];
    for (ndx, state) in self.procs.iter_mut().enumerate() {
      let ndx = AutoNdx(ndx);
      if let Some(Val::List(event)) = state.take_event() {
        if let Some(Val::Sym(event_name)) = event.get(0) {
          if let Some(handler) = self.event_handlers.get(event_name) {
            events.push((event, *handler, ndx));
          }
        }
      }
    }

    for (event, handler, ndx) in events {
      handler(event, self, world, ndx);
    }
  }

  pub fn process_messages(&mut self, world: &mut World) {
    let mut messages = vec![];
    for (ndx, state) in self.procs.iter_mut().enumerate() {
      let ndx = AutoNdx(ndx);
      if let Some(message) = state.message_peek() {
        if let Some(Val::Sym(message_name)) = message.get(0) {
          if let Some(handler) = self.message_handlers.get(message_name) {
            messages.push((message, *handler, ndx));
          }
        }
      }
    }

    for (message, handler, ndx) in messages {
      let result = handler(message, self, world, ndx);
      if let Some(result) = result {
        self.procs[ndx.0].message_return(result);
      }
    }
  }

  pub fn interrupt(&mut self, robo: AutoNdx, message: Val) {
    self.ensure_size(robo.0);
    self.procs[robo.0].interrupt(message);
  }

  pub fn idle(&self, robo: AutoNdx) -> bool {
    if self.procs.len() <= robo.0 {
      return true;
    }
    self.procs[robo.0].finished()
  }
}
