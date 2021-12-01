use eframe::{egui, epi};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DebuggerState {
    label: String,
    pub value: f32,
	
}

impl Default for DebuggerState {
    fn default() -> Self {
		Self {
			label: "he".to_owned(),
			value: 2.7,
		}		
	}
}

pub struct DebuggerStateArc {
	state_arc: Arc<Mutex<DebuggerState>>,
	pub egui_ctx_ref: Option<egui::CtxRef> 
}

impl DebuggerStateArc {
	pub fn new(a: Arc<Mutex<DebuggerState>>) -> Self {
		Self {
			state_arc: a,
			egui_ctx_ref: None
		}
	}
}

impl epi::App for DebuggerStateArc {
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
		self.egui_ctx_ref = Some(egui::CtxRef::clone(_ctx));
    }

   
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { state_arc, egui_ctx_ref: _ } = self;
		let mut state = state_arc.lock().unwrap();
		
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            ui.heading("Side Panel 2");
            ui.heading("Side Panel 3");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                //ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(&mut state.value, 0.0..=10.0).text("value"));

			if ui.button("Increment").clicked() {
                state.value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");            
        });
        
    }
}
