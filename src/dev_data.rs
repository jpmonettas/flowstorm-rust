use std::sync::{Arc, Mutex};
use crate::state::{DebuggerState, ExprTrace, BindTrace, Form, FnCallTrace};
 
pub fn add_nested_let_flow(debugger_state_arc: &Arc<Mutex<DebuggerState>>) {
	let mut state = debugger_state_arc
        .lock()
        .expect("Can't get the lock on state mutex");

    
} 

pub fn add_factorial(debugger_state_arc: &Arc<Mutex<DebuggerState>>) {
	let mut state = debugger_state_arc
        .lock()
        .expect("Can't get the lock on state mutex");
    
	state.add_flow_form(1709,71712880,Form::new("(defn factorial [n] (if (zero? n) 1 (* n (factorial (dec n)))))".to_string()), 1643740412526);
	state.add_fn_call_trace(1709, FnCallTrace::new(71712880,"factorial".to_string(),"[5]".to_string(),1643740412580));
	state.add_bind_trace(1709, BindTrace::new(71712880, "n".to_string(),"5".to_string(),vec![], 1643740412581));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"5".to_string(),vec![3, 1, 1], 1643740412582));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"false".to_string(),vec![3, 1], 1643740412582));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"5".to_string(),vec![3, 3, 1], 1643740412583));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"5".to_string(),vec![3, 3, 2, 1, 1], 1643740412583));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"4".to_string(),vec![3, 3, 2, 1], 1643740412584));
	state.add_fn_call_trace(1709, FnCallTrace::new(71712880,"factorial".to_string(),"[4]".to_string(),1643740412584));
	state.add_bind_trace(1709, BindTrace::new(71712880, "n".to_string(),"4".to_string(),vec![], 1643740412584));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"4".to_string(),vec![3, 1, 1], 1643740412584));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"false".to_string(),vec![3, 1], 1643740412585));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"4".to_string(),vec![3, 3, 1], 1643740412585));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"4".to_string(),vec![3, 3, 2, 1, 1], 1643740412585));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"3".to_string(),vec![3, 3, 2, 1], 1643740412585));
	state.add_fn_call_trace(1709, FnCallTrace::new(71712880,"factorial".to_string(),"[3]".to_string(),1643740412586));
	state.add_bind_trace(1709, BindTrace::new(71712880, "n".to_string(),"3".to_string(),vec![], 1643740412586));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"3".to_string(),vec![3, 1, 1], 1643740412586));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"false".to_string(),vec![3, 1], 1643740412586));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"3".to_string(),vec![3, 3, 1], 1643740412586));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"3".to_string(),vec![3, 3, 2, 1, 1], 1643740412587));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3, 3, 2, 1], 1643740412587));
	state.add_fn_call_trace(1709, FnCallTrace::new(71712880,"factorial".to_string(),"[2]".to_string(),1643740412587));
	state.add_bind_trace(1709, BindTrace::new(71712880, "n".to_string(),"2".to_string(),vec![], 1643740412587));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3, 1, 1], 1643740412588));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"false".to_string(),vec![3, 1], 1643740412588));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3, 3, 1], 1643740412588));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3, 3, 2, 1, 1], 1643740412588));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 3, 2, 1], 1643740412588));
	state.add_fn_call_trace(1709, FnCallTrace::new(71712880,"factorial".to_string(),"[1]".to_string(),1643740412589));
	state.add_bind_trace(1709, BindTrace::new(71712880, "n".to_string(),"1".to_string(),vec![], 1643740412589));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 1, 1], 1643740412589));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"false".to_string(),vec![3, 1], 1643740412589));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 3, 1], 1643740412589));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 3, 2, 1, 1], 1643740412590));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"0".to_string(),vec![3, 3, 2, 1], 1643740412590));
	state.add_fn_call_trace(1709, FnCallTrace::new(71712880,"factorial".to_string(),"[0]".to_string(),1643740412590));
	state.add_bind_trace(1709, BindTrace::new(71712880, "n".to_string(),"0".to_string(),vec![], 1643740412590));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"0".to_string(),vec![3, 1, 1], 1643740412591));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"true".to_string(),vec![3, 1], 1643740412591));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3], 1643740412591));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![], 1643740412591));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 3, 2], 1643740412592));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 3], 1643740412592));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3], 1643740412592));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![], 1643740412592));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"1".to_string(),vec![3, 3, 2], 1643740412592));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3, 3], 1643740412593));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3], 1643740412593));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![], 1643740412593));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"2".to_string(),vec![3, 3, 2], 1643740412593));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"6".to_string(),vec![3, 3], 1643740412594));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"6".to_string(),vec![3], 1643740412594));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"6".to_string(),vec![], 1643740412594));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"6".to_string(),vec![3, 3, 2], 1643740412594));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"24".to_string(),vec![3, 3], 1643740412595));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"24".to_string(),vec![3], 1643740412595));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"24".to_string(),vec![], 1643740412595));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"24".to_string(),vec![3, 3, 2], 1643740412595));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"120".to_string(),vec![3, 3], 1643740412596));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"120".to_string(),vec![3], 1643740412596));
	state.add_exec_trace(1709,ExprTrace::new(71712880,"120".to_string(),vec![], 1643740412596));
	
}
