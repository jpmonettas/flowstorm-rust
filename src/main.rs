#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use eframe::egui::CtxRef;
use eframe::egui::Context;

mod app;

fn main() {
    let debugger_state = app::DebuggerState::default();
	let debugger_state_mutex = Mutex::new(debugger_state);
	let debugger_state_mutex_arc = Arc::new(debugger_state_mutex);

	let thread_ref = Arc::clone(&debugger_state_mutex_arc);

	let dsa = app::DebuggerStateArc::new(Arc::clone(&debugger_state_mutex_arc));
	
	let egui_ctx = match dsa.egui_ctx_ref {
		Some(ref ctx) => Some(CtxRef::clone(&ctx)),
		None      => None,
	};
					
	let _h = thread::spawn (move || {        
		loop {
			{ // this block is so we release the thread_ref mutex loc on evenry iteration
			  // after aquiring it, since the UI is using the same lock on repaint	
				let mut state = thread_ref.lock().unwrap();
				println!("*******{:?}", state);
				state.value += 0.1;
				match egui_ctx {
					Some(ref ctx) => ctx.request_repaint(),
					None      => println!("UI not ready yet"),
				}
			}
            thread::sleep(Duration::from_millis(1000));
		}
		
	});
	
    // h.join().unwrap();
    
    let native_options = eframe::NativeOptions::default();
	eframe::run_native(Box::new(dsa), native_options);
    
}
