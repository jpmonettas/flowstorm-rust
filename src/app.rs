use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DebuggerState {
    pub value: f32,
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self { value: 2.7 }
    }
}

pub struct DebuggerApp {
    state_arc: Arc<Mutex<DebuggerState>>,
    pub ctx_chan_sender: Sender<egui::CtxRef>,
}

impl DebuggerApp {
    pub fn new(
        state_arc: Arc<Mutex<DebuggerState>>,
        ctx_chan_sender: Sender<egui::CtxRef>,
    ) -> Self {
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
        self.ctx_chan_sender
            .send(_ctx.clone())
            .expect("Could not send to context channel during setup");
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        println!("REPAINTING..................");

        let mut state = self
            .state_arc
            .lock()
            .expect("Could not obtain a lock on state");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flowstorm debugger");
            ui.add(egui::Slider::new(&mut state.value, 0.0..=10.0).text("value"));
        });
    }
}
