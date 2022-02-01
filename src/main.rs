#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds #![warn(clippy::all, rust_2018_idioms)] extern crate json; mod views; mod ws;

use epi::NativeOptions;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use egui_glow;

mod lisp_pprinter;
mod lisp_reader;
mod state;
mod views;
mod ws;

mod dev_data;

fn main() {
    let debugger_state_arc = Arc::new(Mutex::new(state::DebuggerState::default()));

	// ONLY FOR TESTING
	crate::dev_data::add_nested_let_flow(&debugger_state_arc); 
	crate::dev_data::add_factorial(&debugger_state_arc); 
	
    let (tx, _rx) = mpsc::channel();

    ws::start_ws_server(Arc::clone(&debugger_state_arc));

    let native_options = NativeOptions::default();

    let dsa = views::DebuggerApp::new(Arc::clone(&debugger_state_arc), tx);
    egui_glow::run(Box::new(dsa), &native_options)
}
