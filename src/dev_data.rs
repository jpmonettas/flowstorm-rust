use std::sync::{Arc, Mutex};
use crate::state::{DebuggerState, ExecTrace, BindTrace, Form};
 
pub fn add_nested_let_flow(debugger_state_arc: &Arc<Mutex<DebuggerState>>) {
	let mut state = debugger_state_arc
        .lock()
        .expect("Can't get the lock on state mutex");

    state.add_flow_form(
        9884,
        427530587,
        Form::new(
            "(let [a {:person/name \"Juan\" :person/age 38 :numbers (range 1000) :more-things [1 2 3 [2 3] {:a #{1 2}} (quote (1 2 3 \"hello\"))]}] (:more-things a))".to_string(),
        ),
        1643391850071,
    );

    state.add_exec_trace(9884, ExecTrace::new(427530587, "(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 ...)".to_string(), vec![1, 1, 5], 1643391850071));
	state.add_bind_trace(9884, BindTrace::new(427530587, "a".to_string(), "{:person/name \"Juan\", :person/age 38, :numbers (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 ...), :more-things [1 2 3 [2 3] {:a #{1 2}} (1 2 3 \"hello\")]}".to_string(), vec![], 1643391850074));
	state.add_exec_trace(9884, ExecTrace::new(427530587, "{:person/name \"Juan\", :person/age 38, :numbers (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 ...), :more-things [1 2 3 [2 3] {:a #{1 2}} (1 2 3 \"hello\")]}".to_string(), vec![2, 1], 1643391850075 ));
	state.add_exec_trace(9884, ExecTrace::new(427530587, "[1 2 3 [2 3] {:a #{1 2}} (1 2 3 \"hello\")]".to_string(), vec![2], 1643391850076 ));
	state.add_exec_trace(9884, ExecTrace::new(427530587, "[1 2 3 [2 3] {:a #{1 2}} (1 2 3 \"hello\")]".to_string(), vec![], 1643391850076 ));
	state.add_exec_trace(9884, ExecTrace::new(427530587, "[1 2 3 [2 3] {:a #{1 2}} (1 2 3 \"hello\")]".to_string(), vec![], 1643391850076 ));

	state.select_flow(9884);		
} 
