use std::collections::HashMap;

use bevy::prelude::{App, Plugin};
use conniver::{Val, State};

use crate::model::{auto::AutoNdx, world::World};

use super::event::EventHandler;

pub struct RS98ProgramPlugin;

impl Plugin for RS98ProgramPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup_program)
      .add_system(update_program);
  }
}

pub fn setup_program(
  
) {

}

pub fn update_program(

) {

}

pub struct ProgramSpace {
  procs: Vec<State>,
}

impl ProgramSpace {
  pub fn new() -> Self {
    Self {
      procs: Vec::new(),
    }
  }

  pub fn set_program(&mut self, robo: AutoNdx, p: Val) {
    if self.procs.len() <= robo.0 {
      self.procs.resize(robo.0 + 1, State::new());
    }
    self.procs[robo.0].set_program(p);
  }

  pub fn update(&mut self, dur: f64) {
    for state in &mut self.procs {
      let result = state.run();
      if let Some(result) = result {
        state.set_program(Val::nil());
      }
    }
  }

  pub fn process_events(&mut self, world: &mut World, event_handlers: &HashMap<String, EventHandler>) {
    let mut events = vec![];
    for (ndx, state) in self.procs.iter_mut().enumerate() {
      let ndx = AutoNdx(ndx);
      if let Some(Val::List(event)) = state.take_event() {
        if let Some(Val::Sym(event_name)) = event.get(0) {
          if let Some(handler) = event_handlers.get(event_name) {
            events.push((event, handler, ndx));
          }
        }
      }
    }

    for (event, handler, ndx) in events {
      handler(event, self, world, ndx);
    }
  }
}
