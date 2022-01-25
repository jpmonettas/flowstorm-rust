use std::collections::HashMap;
use crate::lisp_pprinter::PrintToken;
use crate::lisp_pprinter::style_lisp_form;
use crate::lisp_reader::read_str;
use crate::lisp_reader::PrintableLispForm;

pub type FlowId = u32;
pub type FormId = u32;

#[derive(Debug)]
pub struct Form {
	form_str: String,
	print_tokens: Vec<PrintToken>
}

impl Form {
	pub fn new(form_str: String) -> Self {
		let mut form = read_str(&form_str).unwrap();
		let tokens = style_lisp_form(&mut form, 40);
		Self {
			form_str,
			print_tokens: tokens
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
	// for debugging
	pub fn get_flow(&self, flow_id: FlowId) -> &Flow {
		self.flows.get(&flow_id).unwrap()
	}
	// for debugging
	pub fn get_form_print_tokens (&self, flow_id: FlowId, form_id: FormId) -> &Vec<PrintToken> {
		&self.get_flow(flow_id).forms.get(&form_id).unwrap().print_tokens
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
