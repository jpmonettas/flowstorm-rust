use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum LispForm {
	Atomic(String),
	String(String),
	List(Vec<LispForm>),
	Vector(Vec<LispForm>),
	Set(Vec<LispForm>),
	Map{ keys: Vec<LispForm>, vals: Vec<LispForm>}
}


fn read_delimited_vec(input: &mut Peekable<Chars>, delim: char) -> Vec<LispForm> {
	input.next(); // discard the open delim
	let mut v : Vec<LispForm> = Vec::new();
    
	while let Some(c) = input.peek() {
        if c == &delim {break;}
		
		if let Some(form) = read(input) {
            v.push(form);            
		}

		// TODO: this should be simpler and more general
		// something like while space or comma skip
		if let Some(cc) = input.peek() {
            if cc == &',' {
				input.next(); // discard comma
			}
		}
		if let Some(cc) = input.peek() {
            if cc == &' ' {
				input.next(); // discard whitespace
			}
		}
	}
	input.next(); // discard closing delim
	return v
}

fn read_atomic_token(input: &mut Peekable<Chars>) -> String {
	let mut s = String::new();
	
    while let Some(c) = input.next() {
		s.push(c.clone());

		if let Some(cc) = input.peek() {
			if cc == &' ' || cc == &',' || cc == &')' || cc == &'}' || cc == &']' {
				break;
			}
		} 
	}

	s
}

fn read_string(input: &mut Peekable<Chars>) -> String {
    let mut s = String::new();
	input.next(); // discard first "
    while let Some(c) = input.next() {
        
		if c == '"' {
			break;
		}
		s.push(c.clone());
	}

    s
}

fn read_map(input: &mut Peekable<Chars>) -> (Vec<LispForm>, Vec<LispForm>) {
    let forms: Vec<LispForm> = read_delimited_vec(input, '}');
	let mut keys: Vec<LispForm> = Vec::new();
	let mut vals: Vec<LispForm> = Vec::new();
	let mut i = 0;

    while i<(forms.len()) {
		if let Some(key) = forms.get(i) {
			if let Some(val) = forms.get(i+1) {
				keys.push(key.clone());
				vals.push(val.clone());
			}
		}
		i=i+2;
	}
	(keys, vals)
}

fn read(input: &mut Peekable<Chars>) -> Option<LispForm> {
    if let Some(c) = input.peek() {
        match c {
			'"' => Some(LispForm::String(read_string(input))),
			'(' => Some(LispForm::List(read_delimited_vec(input, ')'))),
			'[' => Some(LispForm::Vector(read_delimited_vec(input, ']'))),
			'#' => {input.next(); Some(LispForm::Set(read_delimited_vec(input, '}')))},
			'{' => {let (keys, vals) = read_map(input); Some(LispForm::Map {keys: keys, vals: vals})},
			_  =>  Some(LispForm::Atomic(read_atomic_token(input))),
		}        
	} else {
		None
	}
}

pub fn read_str(input: &str) -> Option<LispForm> {
	let mut input_iter = input.chars().peekable();
	read(&mut input_iter)
}

fn lisp_form_vec_to_str(v: Vec<LispForm>) -> String {
	v.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(" ")
}

impl ToString for LispForm {
	fn to_string(&self) -> String {
        match self {
			LispForm::String(s) => format!("\"{}\"", s),
			LispForm::Atomic(s) => format!("{}", s),
			LispForm::List(v) => { format!("({})", lisp_form_vec_to_str(v.to_vec()))},
			LispForm::Vector(v) => { format!("[{}]", lisp_form_vec_to_str(v.to_vec()))},
			LispForm::Set(v) => { format!("#{{{}}}", lisp_form_vec_to_str(v.to_vec()))},
			LispForm::Map{keys: keys, vals: vals} => {
				let content = keys.iter().zip(vals).map(|(k, v)| format!("{} {}", k.to_string(), v.to_string())).collect::<Vec<String>>().join(" ");
				format!("{{{}}}", content)
			},
			
		}
	}
}

///////////
// Tests //
///////////

#[test]
fn read_string_test() {
	let mut r_str = String::from("");
	if let Some(form) = read_str("\"this is a string\"") {
		if let LispForm::String(s) = form {
			r_str = s;
		}		
	}
    assert_eq!(r_str , String::from("this is a string"));
}

#[test]
fn read_atomic_token_test() {
	let mut r_str = String::from("");
	if let Some(form) = read_str("some_token") {
		if let LispForm::Atomic(s) = form {
			r_str = s;
		}		
	}
    assert_eq!(r_str , String::from("some_token"));
}

#[test]
fn read_delimited_vec_simple_test() {
	let mut r : Vec<LispForm> = Vec::new();
	if let Some(form) = read_str("[1 2 3 4]") {
		if let LispForm::Vector(childs) = form {
            r = childs;
		}		
	}
    assert_eq!(r.len() , 4);    
}

#[test]
fn read_delimited_vec_nested_test() {
	let mut r : Vec<LispForm> = Vec::new();
	if let Some(form) = read_str("[#{1 something} 8 [2 3] (hello \"world\" 5)]") {
		if let LispForm::Vector(childs) = form {
            r = childs;
		}		
	}
    assert_eq!(r.len() , 4);    
}

#[test]
fn read_str_map_test() {
	let mut rkeys : Vec<LispForm> = Vec::new();
	let mut rvals : Vec<LispForm> = Vec::new();
	if let Some(form) = read_str("{1 2 3 4}") {
		if let LispForm::Map {keys, vals} = form {
			rkeys = keys;
			rvals = vals;
		}		
	}	
    assert_eq!(rkeys.len() , 2);
    assert_eq!(rvals.len() , 2);
	
}

#[test]
fn read_str_code_1_test() {
	let mut r : Vec<LispForm> = Vec::new();
	if let Some(form) = read_str("(defn factorial [n] (if (zero? n) 1 (* n (factorial (dec n)))))") {
		if let LispForm::List(childs) = form {
            r = childs;
		}		
	}
    assert_eq!(r.len() , 4);
}

#[test]
fn read_str_code_2_test() {
	let mut r : Vec<LispForm> = Vec::new();
	if let Some(form) = read_str("(let [a [1 2 3] b {:n/a 1, :c 2}] a)") {
		if let LispForm::List(childs) = form {
            r = childs;
		}		
	}
    assert_eq!(r.len(), 3);
}

#[test]
fn to_string_test() {
    
	let input = "(let [a #{1 2} b [1/3 2] c (\"1\" 2)] (concat a b c))";
    
	if let Some(form) = read_str(input) {
		assert_eq!(form.to_string(), String::from(input));
	} else {
		assert!(false);
	}
    
}

// TODO: string with scaped quotes inside
