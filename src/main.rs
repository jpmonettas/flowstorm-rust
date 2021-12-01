#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use {epi};
use epi::NativeOptions;
use egui_glium;
mod views;

fn main() {
    let debugger_state_arc = Arc::new(Mutex::new(views::DebuggerState::default()));

	let thread_state_ref = Arc::clone(&debugger_state_arc);
    let (tx, rx) = mpsc::channel();
	
	let dsa = views::DebuggerApp::new(Arc::clone(&debugger_state_arc), tx.clone());
    
	let _h = thread::spawn (move || {
		let ctx = rx.recv().unwrap();
		loop {            
			{ // this block is so we release the thread_ref mutex loc on evenry iteration
				// after aquiring it, since the UI is using the same lock on repaint	
				let mut state = thread_state_ref.lock().unwrap();
				println!("*******{:?}", state);
				state.value += 0.1;                
			}
			ctx.request_repaint();
            thread::sleep(Duration::from_millis(1000));
		}
		
	});
	
    // h.join().unwrap();
    
    let native_options = NativeOptions::default();
	egui_glium::run(Box::new(dsa), &native_options)
	//eframe::run_native(Box::new(dsa), native_options);
    
}
