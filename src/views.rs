use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::state::DebuggerState;

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
		let state = state_arc.lock().unwrap();
        let first_flow = state.first_flow();
		egui::CentralPanel::default().show(ctx, |ui| {
			match first_flow {
				Some(f) => ui.heading(format!("Flows {} Forms {} Traces {}",
											  state.flows.len(),
											  f.forms.len(),
				                              f.exec_traces.len())),                
				None => ui.heading("No flows yet"),
			}
            
			//ui.add(egui::Slider::new(&mut state.value, 0..=10).text("value"));
		});
		
	}
}
