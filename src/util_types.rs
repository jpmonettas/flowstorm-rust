use crate::state::{Form, FormId};
use std::collections::HashMap;

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

    pub fn get(&self, form_id: FormId) -> Option<&Form> {
        self.forms_map.get(&form_id)
    }

    pub fn get_mut(&mut self, form_id: FormId) -> Option<&mut Form> {
        self.forms_map.get_mut(&form_id)
    }

    pub fn insert(&mut self, form_id: FormId, form: Form) {
        let timestamp = form.timestamp.clone();
        self.forms_map.insert(form_id, form);
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
            let (_, formId) = self.sorted_forms.order[self.pos];
            self.pos += 1;
            self.sorted_forms.forms_map.get(&formId)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut sfs = SortedForms::new();

        sfs.insert(1, Form::new("Form1".to_string(), 50));
        sfs.insert(2, Form::new("Form2".to_string(), 10));
        sfs.insert(3, Form::new("Form3".to_string(), 70));
        sfs.insert(4, Form::new("Form4".to_string(), 20));
        sfs.insert(5, Form::new("Form5".to_string(), 11));

        assert_eq!(sfs.get(1).unwrap().timestamp, 50);
        assert_eq!(sfs.get(2).unwrap().timestamp, 10);
        assert_eq!(sfs.get(3).unwrap().timestamp, 70);

        let rv = sfs.iter().map(|f| f.timestamp).collect::<Vec<u64>>();
        assert_eq!(rv, vec![10, 11, 20, 50, 70]);
    }
}
