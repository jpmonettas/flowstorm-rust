use crate::lisp_pprinter::style_lisp_form;
use crate::lisp_pprinter::PrintToken;
use crate::lisp_reader::read_str;
use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::HashSet;

pub type FlowId = u32;
pub type FormId = u32;

#[derive(Debug)]
pub struct Form {
    print_tokens: Vec<PrintToken>,
    hot_coords: HashSet<Vec<u16>>,
}

impl Form {
    pub fn new(form_str: String) -> Self {
        let mut form = read_str(&form_str).unwrap();
        let tokens = style_lisp_form(&mut form, 40);
        Self {
            print_tokens: tokens,
            hot_coords: HashSet::new(),
        }
    }

    pub fn add_hot_coord(&mut self, coord: Vec<u16>) {
        self.hot_coords.insert(coord);
    }

    pub fn print_tokens(&self) -> &Vec<PrintToken> {
        &self.print_tokens
    }

    pub fn is_coord_hot(&self, coord: &[u16]) -> bool {
        self.hot_coords.contains(coord)
    }
}

#[derive(Debug, Clone)]
pub struct ExecTrace {
    pub form_id: FormId,
    pub result: String,
    pub coord: Vec<u16>,
    pub timestamp: u64,
}

impl ExecTrace {
    pub fn new(form_id: FormId, result: String, coord: Vec<u16>, timestamp: u64) -> Self {
        Self {
            form_id,
            result,
            coord,
            timestamp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BindTrace {
    pub form_id: FormId,
    pub symbol: String,
    pub value: String,
    pub coord: Vec<u16>,
    pub timestamp: u64,
}

impl BindTrace {
    pub fn new(
        form_id: FormId,
        symbol: String,
        value: String,
        coord: Vec<u16>,
        timestamp: u64,
    ) -> Self {
        Self {
            form_id,
            symbol,
            value,
            coord,
            timestamp,
        }
    }
}

#[derive(Debug)]
pub struct FlowExecution {
    pub traces: Vec<ExecTrace>,
    pub curr_trace_idx: usize,
}

impl FlowExecution {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            curr_trace_idx: 0,
        }
    }

    pub fn add_trace(&mut self, trace: ExecTrace) {
        self.traces.push(trace)
    }

    pub fn step_next(&mut self) {
        if self.curr_trace_idx < self.traces.len() - 1 {
            self.curr_trace_idx += 1;
        }
    }

    pub fn step_back(&mut self) {
        if self.curr_trace_idx > 0 {
            self.curr_trace_idx -= 1;
        }
    }

    pub fn jump_to(&mut self, trace_idx: &usize) {
        self.curr_trace_idx = *trace_idx;
    }

    pub fn is_current_coord_executing(&self, coord: &[u16]) -> bool {
        if let Some(t) = self.traces.get(self.curr_trace_idx) {
            t.coord.iter().eq(coord)
        } else {
            false
        }
    }

    pub fn traces_for_coord(&self, coord: &[u16]) -> Vec<(usize, ExecTrace)> {
        let mut r: Vec<(usize, ExecTrace)> = Vec::new();

        for (idx, t) in self.traces.iter().enumerate() {
            if t.coord.eq(coord) {
                r.push((idx, t.clone()))
            }
        }
        r
    }

    pub fn executing_tarce(&self) -> &ExecTrace {
        self.traces.get(self.curr_trace_idx).unwrap()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flow {
    pub flow_id: FlowId,
    pub forms: HashMap<FormId, Form>,
    pub execution: FlowExecution,
    pub bind_traces: Vec<BindTrace>,
    timestamp: u64,
}

fn is_coord_in_scope(scope_coord: &Vec<u16>, current_coord: &Vec<u16>) -> bool {
    if scope_coord.is_empty() {
        true
    } else if scope_coord.len() > current_coord.len() {
        false
    } else {
        scope_coord.iter().zip(current_coord).all(|(x, y)| x == y)
    }
}

impl Flow {
    pub fn current_locals(&self) -> Vec<(&str, &str)> {
        let curr_trace = self.execution.executing_tarce();
        let mut bindings: HashMap<&str, &str> = HashMap::new();
        for bt in &self.bind_traces {
            if bt.form_id == curr_trace.form_id
                && bt.timestamp <= curr_trace.timestamp
                && is_coord_in_scope(&bt.coord, &curr_trace.coord)
            {
                bindings.insert(bt.symbol.as_str(), bt.value.as_str());
            }
        }
        let mut bindings_vec: Vec<(&str, &str)> = bindings.into_iter().collect();
        bindings_vec.sort_by_key(|t| t.1);
        bindings_vec
    }
}

#[derive(Debug, Default)]
pub struct DebuggerState {
    pub flows: HashMap<FlowId, Flow>,
    pub selected_flow_id: Option<FlowId>,
}

impl DebuggerState {
    pub fn add_flow_form(&mut self, flow_id: FlowId, form_id: FormId, form: Form, timestamp: u64) {
        println!(
            "Adding a flow form {} {} {:?} {}",
            flow_id, form_id, form, timestamp
        );
        if let hash_map::Entry::Vacant(e) = self.flows.entry(flow_id) {
            // Initialize the flow, then add form
            let mut flow = Flow {
                flow_id,
                forms: HashMap::new(),
                execution: FlowExecution::new(),
                bind_traces: Vec::new(),
                timestamp,
            };
            flow.forms.insert(form_id, form);
            e.insert(flow);
        } else {
            // Add form to the flow
            if let Some(flow) = self.flows.get_mut(&flow_id) {
                flow.forms.insert(form_id, form);
            };
        }
    }

    pub fn add_exec_trace(&mut self, flow_id: FlowId, exec_trace: ExecTrace) {
        println!("Adding exec trace {} {:?}", flow_id, exec_trace);
        let form_id = exec_trace.form_id;
        let coord = exec_trace.coord.clone();

        if let Some(flow) = self.flows.get_mut(&flow_id) {
            flow.execution.add_trace(exec_trace);

            if let Some(ref mut form) = flow.forms.get_mut(&form_id) {
                form.add_hot_coord(coord);
            }
        };
    }

    pub fn add_bind_trace(&mut self, flow_id: FlowId, bind_trace: BindTrace) {
        println!("Adding bind trace {} {:?}", flow_id, bind_trace);

        if let Some(flow) = self.flows.get_mut(&flow_id) {
            flow.bind_traces.push(bind_trace);
        };
    }

    pub fn select_flow(&mut self, flow_id: FlowId) {
        self.selected_flow_id = Some(flow_id);
    }

    #[allow(dead_code)] // TODO: remove this
    pub fn selected_flow(&self) -> Option<&Flow> {
        if let Some(selected_flow_id) = self.selected_flow_id {
            self.flows.get(&selected_flow_id)
        } else {
            None
        }
    }

    pub fn selected_flow_mut(&mut self) -> Option<&mut Flow> {
        if let Some(selected_flow_id) = self.selected_flow_id {
            self.flows.get_mut(&selected_flow_id)
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn is_coord_in_scope_test() {
        assert!(is_coord_in_scope(&vec![], &vec![1, 2]));
        assert!(is_coord_in_scope(&vec![1, 2], &vec![1, 2]));
        assert!(is_coord_in_scope(&vec![1, 2], &vec![1, 2, 3]));
        assert!(!is_coord_in_scope(&vec![1, 2, 3], &vec![1, 2]));
    }
}
