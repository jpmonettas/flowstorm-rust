use crate::lisp_pprinter::style_lisp_form;
use crate::lisp_pprinter::PrintToken;
use crate::lisp_reader;
use crate::lisp_reader::{read_str, PrintableLispForm};
use crate::util_types::{CallStackTree, SortedForms};
use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::HashSet;

pub type FlowId = i64;
pub type FormId = i64;
pub type ThreadId = u16;
pub type Coord = Vec<u16>;

#[derive(Debug, Clone)]
pub struct Form {
    pub print_tokens: Vec<PrintToken>,
    pub form_id: FormId,
    pub timestamp: u64,
    pub ns: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprTrace {
    pub form_id: FormId,
    pub result: String,
    pub coord: Coord,
    pub timestamp: u64,
    pub is_outer_form: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCallTrace {
    pub form_id: FormId,
    pub fn_name: String,
    pub fn_ns: String,
    pub args_vec: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecTrace {
    ExprTrace(ExprTrace),
    FnCallTrace(FnCallTrace),
}

#[derive(Debug, Clone)]
pub struct BindTrace {
    pub form_id: FormId,
    pub symbol: String,
    pub value: String,
    pub coord: Coord,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct FlowExecution {
    pub traces: Vec<ExecTrace>,
    pub curr_trace_idx: usize,
}

#[derive(Debug, PartialEq)]
pub enum FlowTool {
    Code,
    CallStack,
}

#[derive(Debug)]
pub struct FlowThread {
    pub thread_id: ThreadId,
    pub execution: FlowExecution,
    pub call_stack_tree: Option<CallStackTree>,
    pub bind_traces: Vec<BindTrace>,
    pub hot_coords: HashMap<FormId, HashSet<Coord>>,
    pub selected_flow_tool: FlowTool,
    pub value_inspector: Option<PrintableLispForm>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Flow {
    pub flow_id: FlowId,
    pub forms: SortedForms,
    pub threads: HashMap<ThreadId, FlowThread>,
    pub selected_thread_id: Option<ThreadId>,
    timestamp: u64,
}

#[derive(Debug, PartialEq)]
pub enum DebuggerTool {
    Flows,
    Refs,
    Taps,
    Timeline,
}

#[derive(Debug)]
pub struct DebuggerState {
    pub flows: HashMap<FlowId, Flow>,
    pub selected_flow_id: Option<FlowId>,
    pub selected_tool: DebuggerTool,
    pub total_trace_count: usize,
}

impl Form {
    pub fn new(form_id: FormId, ns: String, form_str: String, timestamp: u64) -> Self {
        let mut form = read_str(&form_str).unwrap();
        let tokens = style_lisp_form(&mut form, 40);
        Self {
            print_tokens: tokens,
            form_id,
            ns,
            timestamp,
        }
    }
}

impl ExprTrace {
    pub fn new(
        form_id: FormId,
        result: String,
        coord: Vec<u16>,
        is_outer_form: bool,
        timestamp: u64,
    ) -> Self {
        Self {
            form_id,
            result,
            coord,
            is_outer_form,
            timestamp,
        }
    }
}

impl FnCallTrace {
    pub fn new(
        form_id: FormId,
        fn_ns: String,
        fn_name: String,
        args_vec: String,
        timestamp: u64,
    ) -> Self {
        Self {
            form_id,
            fn_name,
            fn_ns,
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

fn is_coord_in_scope(scope_coord: &Coord, current_coord: &Coord) -> bool {
    if scope_coord.is_empty() {
        true
    } else if scope_coord.len() > current_coord.len() {
        false
    } else {
        scope_coord.iter().zip(current_coord).all(|(x, y)| x == y)
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

    pub fn is_current_coord_executing(&self, form_id: FormId, coord: &[u16]) -> bool {
        if let ExecTrace::ExprTrace(et) = self.executing_trace() {
            if et.form_id == form_id {
                et.coord.iter().eq(coord)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn traces_for_coord(&self, form_id: FormId, coord: &[u16]) -> Vec<(usize, ExprTrace)> {
        let mut r: Vec<(usize, ExprTrace)> = Vec::new();

        for (idx, t) in self.traces.iter().enumerate() {
            if let ExecTrace::ExprTrace(et) = t {
                if et.form_id == form_id && et.coord.eq(coord) {
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

impl FlowThread {
    pub fn new(thread_id: ThreadId) -> Self {
        Self {
            thread_id,
            execution: FlowExecution::new(),
            //call_stack_depth: 1,
            call_stack_tree: None,
            bind_traces: Vec::new(),
            hot_coords: HashMap::new(),
            selected_flow_tool: FlowTool::Code,
            value_inspector: None,
        }
    }

    pub fn add_expr_trace(&mut self, expr_trace: ExprTrace) {
        let form_id = expr_trace.form_id;
        let coord = expr_trace.coord.clone();

        if expr_trace.is_outer_form {
            if let Some(ref mut cst) = self.call_stack_tree {
                cst.pop();
            }
        }

        self.execution.add_expr_trace(expr_trace);

        if let hash_map::Entry::Vacant(e) = self.hot_coords.entry(form_id) {
            // if it is the first hot coord for the form, create the set
            let mut hot_coords = HashSet::new();

            hot_coords.insert(coord);
            e.insert(hot_coords);
        } else {
            // else just inssert
            self.hot_coords.get_mut(&form_id).unwrap().insert(coord);
        }
    }

    pub fn add_fn_call_trace(&mut self, fn_call_trace: FnCallTrace) {
        self.execution.add_fn_call_trace(fn_call_trace);

        let trace_idx = self.execution.traces.len() - 1;

        match self.call_stack_tree {
            // initialize the thread call_stack_tree on first fn_call_trace
            None => {
                self.call_stack_tree = Some(CallStackTree::new(trace_idx));
            }
            Some(ref mut cst) => {
                cst.call(trace_idx);
            }
        }
    }

    pub fn add_bind_trace(&mut self, bind_trace: BindTrace) {
        self.bind_traces.push(bind_trace);
    }

    pub fn is_coord_hot(&self, form_id: FormId, coord: &Coord) -> bool {
        match self.hot_coords.get(&form_id) {
            Some(hot_set) => hot_set.contains(coord),
            None => false,
        }
    }

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

    pub fn update_value_inspector(&mut self, value: &str) {
        if let Some(result_pf) = lisp_reader::read_str(value) {
            self.value_inspector = Some(result_pf);
        }
    }

    fn update_value_inspector_with_current_trace(&mut self) {
        if let ExecTrace::ExprTrace(et) = self.execution.executing_trace().clone() {
            self.update_value_inspector(&et.result);
        }
    }

    pub fn step_next(&mut self) {
        self.execution.step_next();
        self.update_value_inspector_with_current_trace();
    }

    pub fn step_back(&mut self) {
        self.execution.step_back();
        self.update_value_inspector_with_current_trace();
    }

    pub fn jump_to(&mut self, trace_idx: &usize) {
        self.execution.jump_to(trace_idx);
        self.update_value_inspector_with_current_trace();
    }
}

impl Flow {
    pub fn thread_ids(&self) -> Vec<ThreadId> {
        self.threads.keys().cloned().collect::<Vec<ThreadId>>()
    }
}

impl DebuggerState {
    pub fn new() -> Self {
        Self {
            flows: HashMap::new(),
            selected_flow_id: None,
            selected_tool: DebuggerTool::Flows,
            total_trace_count: 0,
        }
    }

    pub fn add_flow_form(&mut self, flow_id: FlowId, form_id: FormId, form: Form, timestamp: u64) {
        if let hash_map::Entry::Vacant(e) = self.flows.entry(flow_id) {
            // Initialize the flow, then add form
            let mut flow = Flow {
                flow_id,
                forms: SortedForms::new(),
                threads: HashMap::new(),
                selected_thread_id: None,
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

    pub fn add_exec_trace(&mut self, flow_id: FlowId, thread_id: ThreadId, expr_trace: ExprTrace) {
        // Add the exec trace to the corresponding FlowThread initializing if necesary
        if let Some(flow) = self.flows.get_mut(&flow_id) {
            if let hash_map::Entry::Vacant(e) = flow.threads.entry(thread_id) {
                // first exec_trace of the FlowThread, create a FlowThread, then add the trace
                let mut thread = FlowThread::new(thread_id);

                thread.add_expr_trace(expr_trace);
                e.insert(thread);

                flow.selected_thread_id = Some(thread_id);
            } else {
                // FlowThread created, just add to it
                flow.threads
                    .get_mut(&thread_id)
                    .unwrap()
                    .add_expr_trace(expr_trace);
            }
        } else {
            println!("Unregistered flow_id {} ... skipping trace", flow_id);
        }
    }

    pub fn add_fn_call_trace(
        &mut self,
        flow_id: FlowId,
        thread_id: ThreadId,
        fn_call_trace: FnCallTrace,
    ) {
        if let Some(flow) = self.flows.get_mut(&flow_id) {
            if let hash_map::Entry::Vacant(e) = flow.threads.entry(thread_id) {
                // first fn_call_trace of the FlowThread, create a FlowThread, then add the trace
                let mut thread = FlowThread::new(thread_id);

                thread.add_fn_call_trace(fn_call_trace);
                e.insert(thread);

                flow.selected_thread_id = Some(thread_id);
            } else {
                // FlowThread created, just add to it
                flow.threads
                    .get_mut(&thread_id)
                    .unwrap()
                    .add_fn_call_trace(fn_call_trace);
            }
        } else {
            println!("Unregistered flow_id {} ... skipping trace", flow_id);
        }
    }

    pub fn add_bind_trace(&mut self, flow_id: FlowId, thread_id: ThreadId, bind_trace: BindTrace) {
        if let Some(flow) = self.flows.get_mut(&flow_id) {
            if let hash_map::Entry::Vacant(e) = flow.threads.entry(thread_id) {
                // first fn_call_trace of the FlowThread, create a FlowThread, then add the trace
                let mut thread = FlowThread::new(thread_id);

                thread.add_bind_trace(bind_trace);
                e.insert(thread);

                flow.selected_thread_id = Some(thread_id);
            } else {
                // FlowThread created, just add to it
                flow.threads
                    .get_mut(&thread_id)
                    .unwrap()
                    .add_bind_trace(bind_trace);
            }
        } else {
            println!("Unregistered flow_id {} ... skipping trace", flow_id);
        }
    }

    pub fn flows_ids(&self) -> Vec<FlowId> {
        self.flows.keys().cloned().collect::<Vec<FlowId>>()
    }

    pub fn select_flow(&mut self, flow_id: FlowId) {
        self.selected_flow_id = Some(flow_id);
    }

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

#[cfg(test)]
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
