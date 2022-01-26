#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds #![warn(clippy::all, rust_2018_idioms)] extern crate json; mod views; mod ws;

use epi::NativeOptions;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::state::ExecTrace; // TODO: remove this
use crate::state::Form; // TODO: remove this

mod lisp_pprinter;
mod lisp_reader;
mod state;
mod views;
mod ws;

fn main() {
    let debugger_state_arc = Arc::new(Mutex::new(state::DebuggerState::default()));

    //for debugging
    {
        let mut state = debugger_state_arc
            .lock()
            .expect("Can't get the lock on state mutex");

        state.add_flow_form(
            6356,
            71712880,
            Form::new(
                "(defn factorial [n] (if (zero? n) 1 (* n (factorial (dec n)))))".to_string(),
            ),
            1643132884439,
        );

        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "0".to_string(), vec![3, 1, 1], 1643132884439),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "true".to_string(), vec![3, 1], 1643132884440),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![3], 1643132884440),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![], 1643132884440),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![3, 3, 2], 1643132884441),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![3, 3], 1643132884441),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![3], 1643132884441),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![], 1643132884442),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "1".to_string(), vec![3, 3, 2], 1643132884442),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "2".to_string(), vec![3, 3], 1643132884442),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "2".to_string(), vec![3], 1643132884442),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "2".to_string(), vec![], 1643132884443),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "2".to_string(), vec![3, 3, 2], 1643132884443),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "6".to_string(), vec![3, 3], 1643132884443),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "6".to_string(), vec![3], 1643132884443),
        );
        state.add_exec_trace(
            6356,
            ExecTrace::new(71712880, "6".to_string(), vec![], 1643132884443),
        );

        state.select_flow(6356);
    }

    let (tx, _rx) = mpsc::channel();

    ws::start_ws_server(Arc::clone(&debugger_state_arc));

    let native_options = NativeOptions::default();

    let dsa = views::DebuggerApp::new(Arc::clone(&debugger_state_arc), tx);
    egui_glium::run(Box::new(dsa), &native_options)
}
