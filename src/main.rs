#![forbid(unsafe_code)] #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds #![warn(clippy::all, rust_2018_idioms)] extern crate json; mod views; mod ws;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use epi::NativeOptions;

mod views;
mod ws;
mod state;
mod lisp_parser;

fn main() {
    let debugger_state_arc = Arc::new(Mutex::new(state::DebuggerState::default()));

    let (tx, _rx) = mpsc::channel();
	
    ws::start_ws_server(Arc::clone(&debugger_state_arc));
    
    let native_options = NativeOptions::default();
	
	let dsa = views::DebuggerApp::new(Arc::clone(&debugger_state_arc), tx);
	egui_glium::run(Box::new(dsa), &native_options)    
    
}
