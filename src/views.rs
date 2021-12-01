use {egui, epi};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender};

#[derive(Debug)]
pub struct DebuggerState {
    pub value: f32,
	
}

impl Default for DebuggerState {
    fn default() -> Self {
		Self {            
			value: 2.7,
		}		
	}
}

pub struct DebuggerApp {
	state_arc: Arc<Mutex<DebuggerState>>,
	pub ctx_chan_sender: Sender<egui::CtxRef>,
}

impl DebuggerApp {
	pub fn new(state_arc: Arc<Mutex<DebuggerState>>, ctx_chan_sender: Sender<egui::CtxRef>) -> Self {
		Self {
			state_arc,
            ctx_chan_sender,
		}
	}
}

impl epi::App for DebuggerApp {
    fn name(&self) -> &str {
        "Flowstorm debugger"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let _r = self.ctx_chan_sender.send(egui::CtxRef::clone(_ctx));        
    }

   
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { state_arc, ctx_chan_sender: _ } = self;
		let mut state = state_arc.lock().unwrap();
		
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flowstorm debugger");
			ui.add(egui::Slider::new(&mut state.value, 0.0..=10.0).text("value"));
        });
        
    }
}
