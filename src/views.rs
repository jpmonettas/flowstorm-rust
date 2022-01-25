use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::state::DebuggerState;
use egui::{Ui, TextStyle, Sense, RichText, Layout, Align};
use crate::lisp_pprinter::PrintToken;
use crate::lisp_pprinter::print_tokens_to_str;

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

fn code_block(ui: &mut Ui, print_tokens: &Vec<PrintToken>) {
	let row_height = ui.fonts()[TextStyle::Body].row_height();
    let one_indent = row_height / 2.0;
    
	let initial_size = egui::vec2(
        ui.available_width(),
        ui.spacing().interact_size.y, // Assume there will be
    );

    let layout = Layout::left_to_right()
        .with_main_wrap(true)
        .with_cross_align(Align::BOTTOM);
	
	ui.allocate_ui_with_layout(initial_size, layout, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let row_height = (*ui.fonts())[TextStyle::Body].row_height();
        ui.set_row_height(row_height);

		for t in print_tokens {
			match t {
				PrintToken::String(s) => {
					ui.label(RichText::new(s));
				},
				PrintToken::BlockOpen{val, coord} => {
					ui.label(RichText::new(val));
				},
				PrintToken::BlockClose{val, coord} => {
					ui.label(RichText::new(val));
				},
				PrintToken::Atomic{val, coord} => {
					ui.label(RichText::new(val));
				},
				PrintToken::Space => {
					ui.label(RichText::new(" "));
				},
				PrintToken::Newline => {
					ui.allocate_exact_size(egui::vec2(0.0, row_height), Sense::hover()); // make sure we take up some height
					ui.end_row();
					ui.set_row_height(row_height);
				},
				PrintToken::PrintTokensVec(_) => panic!("all print tokens should be flatten at this stage"),						
			} 
		}
    });
	
}

impl epi::App for DebuggerApp {
	fn name(&self) -> &str {
		"Flowstorm debugger"
	}

	/// Called once before the first frame.
	fn setup(
		&mut self,
		_ctx: &egui::CtxRef,
		_frame: &epi::Frame,
		_storage: Option<&dyn epi::Storage>,
	) {
		let _r = self.ctx_chan_sender.send(egui::CtxRef::clone(_ctx));        
	}

	
	/// Called each time the UI needs repainting, which may be many times per second.
	/// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
	fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
		let Self { state_arc, ctx_chan_sender: _ } = self;
		let state = state_arc.lock().unwrap();
        let first_flow = state.get_flow(6356);
        
		egui::CentralPanel::default().show(ctx, |ui| {
			
			egui::TopBottomPanel::top("top_panel")
				.resizable(true)
				.min_height(32.0)
				.show_inside(ui, |ui| {
                    ui.heading(format!("Flows {} Forms {} Traces {}",
																   state.flows.len(),
																   first_flow.forms.len(),
																   first_flow.exec_traces.len()));
					
				});

			egui::CentralPanel::default().show_inside(ui, |ui| {
				ui.vertical_centered(|ui| {
					code_block(ui, state.get_form_print_tokens(6356, 71712880));                                        
				});
                
			});
			
            
			//ui.add(egui::Slider::new(&mut state.value, 0..=10).text("value"));
		});
		
	}
}
