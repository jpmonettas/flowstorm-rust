use std::collections::HashMap;

pub type FlowId = u32;
pub type FormId = u32;

#[derive(Debug)]
pub struct Form {
	form_str: String
}

impl Form {
	pub fn new(form_str: String) -> Self {
		Self {
			form_str
		}
	}
}

#[derive(Debug)]
pub struct ExecTrace {
	form_id: FormId,
	result: String,
	coord: Vec<u16>,
	timestamp: u64  
}

impl ExecTrace {
	pub fn new(form_id: FormId, result: String, coord: Vec<u16>, timestamp: u64) -> Self {
		Self {
			form_id,
			result,
			coord,
			timestamp
		}
	}
}

#[derive(Debug)]
pub struct Flow {
    pub forms: HashMap<FormId,Form>,
	pub exec_traces: Vec<ExecTrace>,
	curr_exec_trace: u64,
	timestamp: u64
}


#[derive(Debug)]
pub struct DebuggerState {
	pub flows: HashMap<FlowId, Flow>	
}

impl Default for DebuggerState {
	fn default() -> Self {
		Self {            
			flows: HashMap::new(),
		}		
	}
}

impl DebuggerState {
	// This is for debug only
	pub fn first_flow (&self) -> Option<&Flow> {        
		let flows_keys : Vec<&FlowId> = self.flows.keys().collect();
        
		return if !flows_keys.is_empty() {
			let fid = flows_keys[0];
			Some(&self.flows[fid])
		} else {
			None
		}
	}

	pub fn add_flow_form(&mut self, flow_id: FlowId, form_id: FormId, form: Form, timestamp: u64) {
		println!("Adding a flow form {} {} {:?} {}", flow_id, form_id, form, timestamp);
		if self.flows.contains_key(&flow_id) {
			// Add form to the flow
			if let Some(f) = self.flows.get_mut(&flow_id) {
				f.forms.insert(form_id, form);
			};
		} else {
			// Initialize the flow, then add form
			let mut flow = Flow {
				forms: HashMap::new(),
				exec_traces: Vec::new(),
				curr_exec_trace: 0,
				timestamp
			};
            flow.forms.insert(form_id, form);
			self.flows.insert(flow_id, flow);
		}
	}
	
	pub fn add_exec_trace(&mut self, flow_id: FlowId, exec_trace: ExecTrace){
		println!("Adding exec trace {} {:?}", flow_id, exec_trace);
		if let Some(f) = self.flows.get_mut(&flow_id) {
				f.exec_traces.push(exec_trace);
			};
	}
}
