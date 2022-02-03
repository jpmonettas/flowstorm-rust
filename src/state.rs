use crate::lisp_pprinter::style_lisp_form;
use crate::lisp_pprinter::PrintToken;
use crate::lisp_reader::read_str;
use crate::util_types::SortedForms;
use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::HashSet;

pub type FlowId = i64;
pub type FormId = i64;

#[derive(Debug)]
pub struct Form {
    print_tokens: Vec<PrintToken>,
    hot_coords: HashSet<Vec<u16>>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ExprTrace {
    pub form_id: FormId,
    pub result: String,
    pub coord: Vec<u16>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct FnCallTrace {
    pub form_id: FormId,
    pub fn_name: String,
    pub args_vec: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum ExecTrace {
    ExprTrace(ExprTrace),
    FnCallTrace(FnCallTrace),
}

#[derive(Debug, Clone)]
pub struct BindTrace {
    pub form_id: FormId,
    pub symbol: String,
    pub value: String,
    pub coord: Vec<u16>,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct FlowExecution {
    pub traces: Vec<ExecTrace>,
    pub curr_trace_idx: usize,
}

#[derive(Debug)]
pub struct Flow {
    pub flow_id: FlowId,
    pub forms: SortedForms,
    pub execution: FlowExecution,
    pub bind_traces: Vec<BindTrace>,
    timestamp: u64,
}

#[derive(Debug, Default)]
pub struct DebuggerState {
    pub flows: HashMap<FlowId, Flow>,
    pub selected_flow_id: Option<FlowId>,
}

impl Form {
    pub fn new(form_str: String, timestamp: u64) -> Self {
        let mut form = read_str(&form_str).unwrap();
        let tokens = style_lisp_form(&mut form, 40);
        Self {
            print_tokens: tokens,
            hot_coords: HashSet::new(),
            timestamp,
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

impl ExprTrace {
    pub fn new(form_id: FormId, result: String, coord: Vec<u16>, timestamp: u64) -> Self {
        Self {
            form_id,
            result,
            coord,
            timestamp,
        }
    }
}

impl FnCallTrace {
    pub fn new(form_id: FormId, fn_name: String, args_vec: String, timestamp: u64) -> Self {
        Self {
            form_id,
            fn_name,
            args_vec,
            timestamp,
        }
    }
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

impl FlowExecution {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            curr_trace_idx: 0,
        }
    }

    pub fn add_fn_call_trace(&mut self, trace: FnCallTrace) {
        self.traces.push(ExecTrace::FnCallTrace(trace));
    }
    pub fn add_expr_trace(&mut self, trace: ExprTrace) {
        self.traces.push(ExecTrace::ExprTrace(trace));
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
        if let ExecTrace::ExprTrace(et) = self.executing_trace() {
            et.coord.iter().eq(coord)
        } else {
            false
        }
    }

    pub fn traces_for_coord(&self, coord: &[u16]) -> Vec<(usize, ExprTrace)> {
        let mut r: Vec<(usize, ExprTrace)> = Vec::new();

        for (idx, t) in self.traces.iter().enumerate() {
            if let ExecTrace::ExprTrace(et) = t {
                if et.coord.eq(coord) {
                    r.push((idx, et.clone()))
                }
            }
        }
        r
    }

    pub fn executing_trace(&self) -> &ExecTrace {
        &self.traces[self.curr_trace_idx]
    }
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
        let curr_trace = self.execution.executing_trace();
        let mut bindings: HashMap<&str, &str> = HashMap::new();
        for bt in &self.bind_traces {
            if let ExecTrace::ExprTrace(et) = curr_trace {
                if bt.form_id == et.form_id
                    && bt.timestamp <= et.timestamp
                    && is_coord_in_scope(&bt.coord, &et.coord)
                {
                    bindings.insert(bt.symbol.as_str(), bt.value.as_str());
                }
            }
        }
        let mut bindings_vec: Vec<(&str, &str)> = bindings.into_iter().collect();
        bindings_vec.sort_by_key(|t| t.1);
        bindings_vec
    }
}

impl DebuggerState {
    pub fn add_flow_form(&mut self, flow_id: FlowId, form_id: FormId, form: Form, timestamp: u64) {
        // println!(
        //     "Adding a flow form {} {} {:?} {}",
        //     flow_id, form_id, form, timestamp
        // );
        if let hash_map::Entry::Vacant(e) = self.flows.entry(flow_id) {
            // Initialize the flow, then add form
            let mut flow = Flow {
                flow_id,
                forms: SortedForms::new(),
                execution: FlowExecution::new(),
                bind_traces: Vec::new(),
                timestamp,
            };
            flow.forms.insert(form_id, form);
            e.insert(flow);

            // make this flow selected
            self.selected_flow_id = Some(flow_id);
        } else {
            // Add form to the flow
            if let Some(flow) = self.flows.get_mut(&flow_id) {
                flow.forms.insert(form_id, form);
            };
        }
    }

    pub fn add_exec_trace(&mut self, flow_id: FlowId, expr_trace: ExprTrace) {
        // println!("Adding exec trace {} {:?}", flow_id, expr_trace);
        let form_id = expr_trace.form_id;
        let coord = expr_trace.coord.clone();

        if let Some(flow) = self.flows.get_mut(&flow_id) {
            flow.execution.add_expr_trace(expr_trace);

            if let Some(ref mut form) = flow.forms.get_mut(form_id) {
                form.add_hot_coord(coord);
            }
        };
    }

    pub fn add_fn_call_trace(&mut self, flow_id: FlowId, fn_call_trace: FnCallTrace) {
        if let Some(flow) = self.flows.get_mut(&flow_id) {
            flow.execution.add_fn_call_trace(fn_call_trace);
        }
    }

    pub fn add_bind_trace(&mut self, flow_id: FlowId, bind_trace: BindTrace) {
        // println!("Adding bind trace {} {:?}", flow_id, bind_trace);

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
