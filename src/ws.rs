use crate::state::DebuggerState;
use crate::state::Form;
use crate::state::{BindTrace, ExprTrace, FnCallTrace};
use json::JsonValue;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::accept;

fn print_state_change_code(s: String) {
    if false {
        println!("{}", s);
    }
}

fn u16_from_json_value(obj: JsonValue) -> u16 {
    let res: f64 = if let JsonValue::Number(n) = obj {
        f64::from(n)
    } else {
        panic!("json value is not a number")
    };
    res as u16
}

fn i64_from_json_value(obj: JsonValue) -> i64 {
    let res: f64 = if let JsonValue::Number(n) = obj {
        f64::from(n)
    } else {
        panic!("json value is not a number")
    };
    res as i64
}

fn u64_from_json_value(obj: JsonValue) -> u64 {
    let res: f64 = if let JsonValue::Number(n) = obj {
        f64::from(n)
    } else {
        panic!("json value is not a number")
    };
    return res as u64;
}

fn string_from_json_value(obj: JsonValue) -> String {
    return match &obj {
        JsonValue::String(fs) => String::from(fs),
        JsonValue::Short(fs) => fs.to_string(),
        _ => panic!("json value not a string"),
    };
}

fn bool_from_json_value(obj: JsonValue) -> bool {
    return match &obj {
        JsonValue::Boolean(b) => b.clone(),
        _ => false,
    };
}

fn process_form_init_trace(state_ref: &Arc<Mutex<DebuggerState>>, obj: &JsonValue) {
    let flow_id = i64_from_json_value(obj["flow-id"].clone());
    let form_id = i64_from_json_value(obj["form-id"].clone());
    let timestamp = u64_from_json_value(obj["timestamp"].clone());
    let form_str = string_from_json_value(obj["form"].clone());
    let form_ns = string_from_json_value(obj["ns"].clone());

    print_state_change_code(format!(
        "state.add_flow_form({},{},Form::new({}, \"{}\".to_string(), r#\"{}\"#.to_string(), {}), {});",
        flow_id, form_id, form_id, form_ns, &form_str, timestamp, timestamp
    ));

    let form = Form::new(form_id, form_ns, form_str, timestamp);

    let mut state = state_ref.lock().expect("Can't get the lock on state mutex");

    state.add_flow_form(flow_id, form_id, form, timestamp);
    state.total_trace_count += 1;
}

fn process_form_add_trace(state_ref: &Arc<Mutex<DebuggerState>>, obj: &JsonValue) {
    let flow_id = i64_from_json_value(obj["flow-id"].clone());
    let form_id = i64_from_json_value(obj["form-id"].clone());
    let coord: Vec<u16> = if let JsonValue::Array(v) = &obj["coor"] {
        v.iter()
            .map(|c: &JsonValue| -> u16 { u16_from_json_value(c.clone()) })
            .collect()
    } else {
        panic!("coor json value is not an array");
    };
    let thread_id = u16_from_json_value(obj["thread-id"].clone());
    let result = string_from_json_value(obj["result"].clone());
    let is_outer_form = bool_from_json_value(obj["outer-form?"].clone());
    let timestamp = u64_from_json_value(obj["timestamp"].clone());

    // TODO: handle the :err filed

    print_state_change_code(format!(
        "state.add_exec_trace({},{},ExprTrace::new({},r#\"{}\"#.to_string(),vec!{:?}, {}, {}));",
        flow_id, thread_id, form_id, &result, &coord, is_outer_form, timestamp
    ));

    let trace = ExprTrace::new(form_id, result, coord, is_outer_form, timestamp);

    let mut state = state_ref.lock().expect("Can't get the lock on state mutex");
    state.add_exec_trace(flow_id, thread_id, trace);
    state.total_trace_count += 1;
}

fn process_fn_call_trace(state_ref: &Arc<Mutex<DebuggerState>>, obj: &JsonValue) {
    let flow_id = i64_from_json_value(obj["flow-id"].clone());
    let form_id = i64_from_json_value(obj["form-id"].clone());

    let fn_ns = string_from_json_value(obj["fn-ns"].clone());
    let fn_name = string_from_json_value(obj["fn-name"].clone());
    let args_vec = string_from_json_value(obj["args-vec"].clone());
    let timestamp = u64_from_json_value(obj["timestamp"].clone());
    let thread_id = u16_from_json_value(obj["thread-id"].clone());

    print_state_change_code(format!("state.add_fn_call_trace({},{}, FnCallTrace::new({},r#\"{}\"#.to_string(), \"{}\".to_string() ,r#\"{}\"#.to_string(),{}));", flow_id, thread_id, form_id, &fn_ns, &fn_name, &args_vec, timestamp));

    let trace = FnCallTrace::new(form_id, fn_ns, fn_name, args_vec, timestamp);
    let mut state = state_ref.lock().expect("Can't get the lock on state mutex");
    state.add_fn_call_trace(flow_id, thread_id, trace);
    state.total_trace_count += 1;
}

fn process_form_add_bind_trace(state_ref: &Arc<Mutex<DebuggerState>>, obj: &JsonValue) {
    let flow_id = i64_from_json_value(obj["flow-id"].clone());
    let form_id = i64_from_json_value(obj["form-id"].clone());
    let coord: Vec<u16> = if let JsonValue::Array(v) = &obj["coor"] {
        v.iter()
            .map(|c: &JsonValue| -> u16 { u16_from_json_value(c.clone()) })
            .collect()
    } else {
        panic!("coor json value is not an array");
    };

    let thread_id = u16_from_json_value(obj["thread-id"].clone());
    let symbol = string_from_json_value(obj["symbol"].clone());
    let value = string_from_json_value(obj["value"].clone());

    let timestamp = u64_from_json_value(obj["timestamp"].clone());

    print_state_change_code(format!("state.add_bind_trace({}, {}, BindTrace::new({}, r#\"{}\"#.to_string(),r#\"{}\"#.to_string(),vec!{:?}, {}));", flow_id, thread_id, form_id, &symbol, &value, &coord, timestamp));
    let trace = BindTrace::new(form_id, symbol, value, coord, timestamp);

    let mut state = state_ref.lock().expect("Can't get the lock on state mutex");
    state.add_bind_trace(flow_id, thread_id, trace);
    state.total_trace_count += 1;
}

pub fn start_ws_server(debugger_state_arc: Arc<Mutex<DebuggerState>>) {
    thread::spawn(move || {
        let server = TcpListener::bind("127.0.0.1:7722").expect("Couldn't bind tcp listener");
        for stream in server.incoming() {
            let thread_state_ref = Arc::clone(&debugger_state_arc);
            thread::spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                println!("Got a connection!!!");
                loop {
                    let msg = websocket
                        .read_message()
                        .expect("Couldn't read socket message");

                    if msg.is_text() {
                        //println!("Message {:?}", msg);

                        let parsed = json::parse(msg.to_text().unwrap())
                            .expect("Couldn't parse the message");

                        if let JsonValue::Array(v) = parsed {
                            let command = &v[0];
                            let obj = &v[1];

                            if let JsonValue::Short(c) = command {
                                match c.as_ref() {
                                    "init-trace" => process_form_init_trace(&thread_state_ref, obj),
                                    "fn-call-trace" => {
                                        process_fn_call_trace(&thread_state_ref, obj)
                                    }
                                    "exec-trace" => process_form_add_trace(&thread_state_ref, obj),
                                    "bind-trace" => {
                                        process_form_add_bind_trace(&thread_state_ref, obj)
                                    }
                                    _ => println!("WARNING! Unhandled command {}", c),
                                }
                            } else {
                                println!("WARNING! Command {} is not a string ", command);
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
