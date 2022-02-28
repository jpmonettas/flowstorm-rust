use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::state::{Form, FormId,ExecTrace, FnCallTrace};

#[derive(Debug)]
pub struct SortedForms {
    forms_map: HashMap<FormId, Form>,
    order: Vec<(u64, FormId)>,
}

pub struct SortedFormsIter<'a> {
    sorted_forms: &'a SortedForms,
    pos: usize,
}

// Provide fast access by id, and fast sorted by form.timestamp iteration
// with not so good insertion, wich doesn't matter because we don't insert
// too many forms for a flow
impl SortedForms {
    pub fn new() -> Self {
        Self {
            forms_map: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn get(&self, form_id: &FormId) -> Option<&Form> {
        self.forms_map.get(form_id)
    }

	#[allow(dead_code)]
    pub fn get_mut(&mut self, form_id: &FormId) -> Option<&mut Form> {
        self.forms_map.get_mut(form_id)
    }

    pub fn insert(&mut self, form_id: FormId, form: Form) {
		if let std::collections::hash_map::Entry::Vacant(e) = self.forms_map.entry(form_id) {
            let timestamp = form.timestamp;
            e.insert(form);
            let idx_result = self
                .order
                .as_slice()
                .binary_search_by(|(ts, _)| ts.cmp(&timestamp).reverse());

            match idx_result {
                Ok(idx) => {
                    self.order.insert(idx, (timestamp, form_id));
                }
                Err(idx) => {
                    self.order.insert(idx, (timestamp, form_id));
                }
            }
        }
    }

	#[allow(dead_code)]
    pub fn iter(&self) -> SortedFormsIter { 
        SortedFormsIter {
            sorted_forms: self,
            pos: 0,
        }
    }
}

impl<'a> Iterator for SortedFormsIter<'a> {
    type Item = &'a Form;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.sorted_forms.order.len() {
            let (_, form_id) = self.sorted_forms.order[self.pos];
            self.pos += 1;
            self.sorted_forms.forms_map.get(&form_id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct CallStackTreeNode<'a> {
	pub fn_call: &'a FnCallTrace,
	pub childs: Vec<Arc<Mutex<CallStackTreeNode<'a>>>>
}

#[derive(Debug)]
pub struct CallStackTree<'a> {
	root: Arc<Mutex<CallStackTreeNode<'a>>>,
	stack: Vec<Arc<Mutex<CallStackTreeNode<'a>>>>	
}

impl<'a> CallStackTree<'a> {
	pub fn new(root_e: &'a FnCallTrace) -> Self {        
		let mut tree = Self {
			root: Arc::new(Mutex::new(CallStackTreeNode {
				fn_call: root_e,
				childs: vec![]
			})),
			stack: vec![]
		};
        
		tree.stack.push(Arc::clone(&tree.root));
        tree
	}

	fn call(&mut self, e: &'a FnCallTrace) {
        
		let node_pointer = Arc::new(Mutex::new(CallStackTreeNode {
			fn_call: e,
			childs: vec![]
		}));
        
		let stack_node_pointer = Arc::clone(&node_pointer);

		// add the node to current function childs
		{
			let mut curr_func_pointer = self.stack.as_slice().last().expect("Stack shouldn't be empty").lock().unwrap();
			curr_func_pointer.childs.push(node_pointer);
		}
        
		self.stack.push(stack_node_pointer);
       
	}

	fn pop(&mut self) {
        self.stack.pop();
	}

	pub fn add_trace(&mut self, trace: &'a ExecTrace) {
		match trace {
			ExecTrace::FnCallTrace(fct) => self.call(fct),
			
			ExecTrace::ExprTrace(et) => if et.is_outer_form {
				self.pop();
			},
		};
	}
	
}


#[cfg(test)]
mod tests {
    use super::*;
	use crate::state::{ExprTrace};
	
    #[test]
    fn sorted_forms_basic_test() {
        let mut sfs = SortedForms::new();

        sfs.insert(1, Form::new(1, "testns".to_string(), "Form1".to_string(), 50));
        sfs.insert(2, Form::new(2, "testns".to_string(), "Form2".to_string(), 10));
        sfs.insert(3, Form::new(3, "testns".to_string(), "Form3".to_string(), 70));
        sfs.insert(4, Form::new(4, "testns".to_string(), "Form4".to_string(), 20));
        sfs.insert(5, Form::new(5, "testns".to_string(), "Form5".to_string(), 11));

        assert_eq!(sfs.get(&1).unwrap().timestamp, 50);
        assert_eq!(sfs.get(&2).unwrap().timestamp, 10);
        assert_eq!(sfs.get(&3).unwrap().timestamp, 70);

        let rv = sfs.iter().map(|f| f.timestamp).collect::<Vec<u64>>();
        assert_eq!(rv, vec![70, 50, 20, 11, 10]);
    }

    #[test]
	fn callstack_tree_test() {
		let first_trace = FnCallTrace {form_id: 0, fn_name: "fn1".to_string(), fn_ns: "ns1".to_string(), args_vec: "[]".to_string(), timestamp: 1};

		let rest_traces = vec![
            
			ExecTrace::FnCallTrace(FnCallTrace {form_id: 0, fn_name: "fn1-1".to_string(), fn_ns: "ns1".to_string(), args_vec: "[]".to_string(), timestamp: 1}),
			ExecTrace::ExprTrace(ExprTrace{form_id: 0, result:"R1-1".to_string(), coord: vec![], timestamp: 1, is_outer_form: true}),
			
			ExecTrace::FnCallTrace(FnCallTrace {form_id: 0, fn_name: "fn1-2".to_string(), fn_ns: "ns1".to_string(), args_vec: "[]".to_string(), timestamp: 1}),
			ExecTrace::ExprTrace(ExprTrace{form_id: 0, result:"R1-2".to_string(), coord: vec![], timestamp: 1, is_outer_form: true}),

			ExecTrace::FnCallTrace(FnCallTrace {form_id: 0, fn_name: "fn1-3".to_string(), fn_ns: "ns1".to_string(), args_vec: "[]".to_string(), timestamp: 1}),
			ExecTrace::FnCallTrace(FnCallTrace {form_id: 0, fn_name: "fn1-3-1".to_string(), fn_ns: "ns1".to_string(), args_vec: "[]".to_string(), timestamp: 1}),
			ExecTrace::ExprTrace(ExprTrace{form_id: 0, result:"R1-3-1".to_string(), coord: vec![], timestamp: 1, is_outer_form: true}),
			ExecTrace::ExprTrace(ExprTrace{form_id: 0, result:"R1-3".to_string(), coord: vec![], timestamp: 1, is_outer_form: true}),
			
			ExecTrace::ExprTrace(ExprTrace{form_id: 0, result:"R1".to_string(), coord: vec![], timestamp: 1, is_outer_form: true}),
		];

        
		let mut callstack_tree: CallStackTree = CallStackTree::new(&first_trace);

        for t in &rest_traces {
			callstack_tree.add_trace(t);
		}
		
		// Just checking the structure by eye
		// TODO: make a proper test
		println!("{:#?}", &callstack_tree);        
		assert!(true);
		
		
	}
}
