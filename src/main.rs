#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds #![warn(clippy::all, rust_2018_idioms)] extern crate json; mod views; mod ws;
#![allow(clippy::collapsible_else_if)]
// FOR LINUX PERF
// sudo echo -1 > /proc/sys/kernel/perf_event_paranoid
// cargo flamegraph --dev
// OR
// cargo flamegraph --dev

use epi::NativeOptions;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

mod lisp_pprinter;
mod lisp_reader;
mod state;
mod util_types;
mod views;
mod ws;

mod dev_data;

fn main() {
    let debugger_state_arc = Arc::new(Mutex::new(state::DebuggerState::new()));

    // ONLY FOR TESTING
    // crate::dev_data::add_nested_let_flow(&debugger_state_arc);
    //crate::dev_data::add_factorial(&debugger_state_arc);
    //crate::dev_data::add_parallel(&debugger_state_arc);
    //crate::dev_data::add_cljs_compiler_1(&debugger_state_arc);
    //crate::dev_data::fn_call(&debugger_state_arc);

    let (tx, _rx) = mpsc::channel();

    ws::start_ws_server(Arc::clone(&debugger_state_arc));

    let native_options = NativeOptions::default();

    let dsa = views::DebuggerApp::new(Arc::clone(&debugger_state_arc), tx);
    egui_glow::run(Box::new(dsa), &native_options)
}
