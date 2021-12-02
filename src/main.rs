// #![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(rust_2018_idioms)]

mod app;

use app::DebuggerApp;
use egui::CtxRef;
use epi::NativeOptions;
use futures::channel::oneshot;
use futures::executor;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    executor::block_on(async {
        let debugger_state_arc = Arc::new(Mutex::new(app::DebuggerState::default()));

        let thread_state_ref = Arc::clone(&debugger_state_arc);
        // let (tx, rx) = mpsc::channel();

        let (sender, receiver) = oneshot::channel::<CtxRef>();

        let dsa = DebuggerApp::new(Arc::clone(&debugger_state_arc), sender);

        let _h = thread::spawn(|| async move {
            let ctx = receiver.await.unwrap();

            println!("CTX received from setup");

            loop {
                let mut state = thread_state_ref.lock().unwrap();
                println!("*******{:?}", state);
                state.value += 0.1;
                // drop mutex guard explicitely
                // so we release the thread_ref mutex loc on every iteration
                // after aquiring it, since the UI is using the same lock on repaint
                drop(state);

                println!("Painting with CTX");

                ctx.request_repaint();
                thread::sleep(Duration::from_millis(1000));
            }
        });

        // println!("@@@ before main thread runs");

        // _h.join().unwrap().await;

        let native_options = NativeOptions::default();
        egui_glium::run(Box::new(dsa), &native_options)
        //eframe::run_native(Box::new(dsa), native_options);
    });
}
