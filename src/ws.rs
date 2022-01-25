use std::sync::{Arc, Mutex};
use std::thread;
use std::net::TcpListener;
use tungstenite::accept;
use json::JsonValue;
use crate::state::DebuggerState;
use crate::state::ExecTrace;
use crate::state::Form;

// :flow-storm/init-trace
// - :flow-id [REQ]
// - :form-id [REQ]
// - :form-flow-id [REQ]
// - :form [REQ]
// - :fixed-flow-id-starter? [OPT] Signals that this is the starting trace of a fixed flow-id trace.
// - :args-vec [OPT]
// - :fn-name [OPT]
// - :timestamp [REQ]

fn u16_from_json_value (obj : JsonValue) -> u16 {
	let res : f64 = if let JsonValue::Number(n) = obj { f64::from(n) } else { panic!("json value is not a number") };
	return res as u16;
}

fn u32_from_json_value (obj : JsonValue) -> u32 {
	let res : f64 = if let JsonValue::Number(n) = obj { f64::from(n) } else { panic!("json value is not a number") };
	return res as u32;
}

fn u64_from_json_value (obj : JsonValue) -> u64 {
	let res : f64 = if let JsonValue::Number(n) = obj { f64::from(n) } else { panic!("json value is not a number") };
	return res as u64;
}

fn string_from_json_value (obj : JsonValue) -> String {
	return match &obj {
		JsonValue::String(fs) => String::from(fs),
		JsonValue::Short(fs) => fs.to_string(),
		_ => panic!("json value not a string"),
	}
}

fn process_form_init_trace (state_ref: &Arc<Mutex<DebuggerState>>, obj : &JsonValue) {
    
	let flow_id = u32_from_json_value(obj["flow-id"].clone());
	let form_id = u32_from_json_value(obj["form-id"].clone());
	let timestamp = u64_from_json_value(obj["timestamp"].clone());
	let form_str = string_from_json_value(obj["form"].clone());
   
	let form = Form::new(form_str);

	let mut state = state_ref.lock().expect("Can't get the lock on state mutex");
    state.add_flow_form(flow_id, form_id, form, timestamp);
	
    
}

// :flow-storm/add-trace
// - :flow-id [REQ]
// - :form-id [REQ]
// - :form-flow-id [REQ]
// - :coor [REQ]
// - :result [REQ]
// - :err [OPT]  A map like {:error/message "..."} in case a exception ocurred evaluating this form. The :result is not present when this key is.
// - :timestamp [REQ]

fn process_form_add_trace (state_ref: &Arc<Mutex<DebuggerState>>, obj : &JsonValue) {
    
	let flow_id = u32_from_json_value(obj["flow-id"].clone());
	let form_id = u32_from_json_value(obj["form-id"].clone());
	let coord : Vec<u16> = if let JsonValue::Array(v) = &obj["coor"] {
		v.iter()
			.map(| c : &JsonValue | -> u16 {
				u16_from_json_value(c.clone())
			})
			.collect()
	} else {
		panic!("coor json value is not an array");
	};
    
	let result = string_from_json_value(obj["result"].clone());
	let timestamp = u64_from_json_value(obj["timestamp"].clone());

	// TODO: handle the :err filed
	
	let trace = ExecTrace::new (form_id, result, coord, timestamp);
	
    let mut state = state_ref.lock().expect("Can't get the lock on state mutex");
	state.add_exec_trace(flow_id, trace);
	
    
}


// :flow-storm/add-bind-trace
// - :flow-id [REQ]
// - :form-id [REQ]
// - :form-flow-id [REQ]
// - :coor [REQ]
// - :symbol [REQ]
// - :value [REQ]
// - :timestamp [REQ]

// :flow-storm/ref-init-trace 
// - :ref-id [REQ]
// - :ref-name [OPT]
// - :init-val [REQ]
// - :timestamp [REQ]

// :flow-storm/ref-trace
// - :ref-id [REQ]
// - :patch [REQ]
// - :timestamp [REQ]


// :flow-storm/tap-trace 
// - :tap-id [REQ]
// - :tap-name [OPT]
// - :value [REQ]
// - :timestamp [REQ]

pub fn start_ws_server (debugger_state_arc : Arc<Mutex<DebuggerState>>) {
	thread::spawn( move || {
		let server = TcpListener::bind("127.0.0.1:7722").expect("Couldn't bind tcp listener");
		for stream in server.incoming() {
			let thread_state_ref = Arc::clone(&debugger_state_arc);
			thread::spawn (move || {                
				let mut websocket = accept(stream.unwrap()).unwrap();
				println!("Got a connection!!!");
				loop {
					let msg = websocket.read_message().expect("Couldn't read socket message");

					if msg.is_text() {

						println!("Message {:?}", msg);

						let parsed = json::parse(msg.to_text().unwrap()).expect("Couldn't parse the message");
						
						if let JsonValue::Array(v) = parsed {
							let command = &v[0];	
							let obj = &v[1];

							println!("Command {:?} !", command);
							if let JsonValue::Short(c) = command {                                
                                match c.as_ref() {
									"flow-storm/init-trace" => process_form_init_trace(&thread_state_ref, obj),
									"flow-storm/add-trace" =>  process_form_add_trace (&thread_state_ref, obj),
									_ => println!("Unhandled command {}", c),
								}
							} else {
								println!("Command {} is not a string ", command);
							}
						}
					} else {
						
						println!("Got something that isn't text in websocket");
						
					}
                 
				}
			});
		}
	});
	
}

