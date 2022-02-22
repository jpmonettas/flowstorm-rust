// TODO: string with scaped quotes inside
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum PrintStyle {
    PairsBlock,
    Binding,
    Defn,
    Standard,
    Linear,
    Unstyled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrintableLispForm {
    Atomic(String, Vec<u16>),
    String(String),
    List {
        childs: Vec<PrintableLispForm>,
        style: PrintStyle,
        coord: Vec<u16>,
    },
    Vector {
        childs: Vec<PrintableLispForm>,
        style: PrintStyle,
        coord: Vec<u16>,
    },
    Set {
        childs: Vec<PrintableLispForm>,
        style: PrintStyle,
        coord: Vec<u16>,
    },
    Map {
        keys: Vec<PrintableLispForm>,
        vals: Vec<PrintableLispForm>,
        style: PrintStyle,
        coord: Vec<u16>,
    },
	Regexp(String),
	Tagged {
		tag: String,
		form: Box<PrintableLispForm>,
		coord: Vec<u16>,
	}
}

fn read_delimited_vec(
    input: &mut Peekable<Chars>,
    delim: char,
    curr_coord: &Vec<u16>,
) -> Vec<PrintableLispForm> {
    input.next(); // discard the open delim
    let mut v: Vec<PrintableLispForm> = Vec::new();
    let mut form_idx = 0;
    while let Some(c) = input.peek() {
        if c == &delim {
            break;
        }

        let mut form_coord = curr_coord.clone();
        form_coord.push(form_idx);
        if let Some(form) = read(input, &form_coord) {
            v.push(form);
            form_idx += 1;
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

    return v;
}

fn read_atomic_token(input: &mut Peekable<Chars>) -> String {
    let mut s = String::new();

    while let Some(c) = input.next() {
        s.push(c.clone());

        if let Some(cc) = input.peek() {
            if cc == &' ' || cc == &',' || cc == &')' || cc == &'}' || cc == &']' || cc == &'(' || cc == &'{' || cc == &'[' {
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

fn read_map(
    input: &mut Peekable<Chars>,
    curr_coord: &Vec<u16>,
) -> (Vec<PrintableLispForm>, Vec<PrintableLispForm>) {
    // TODO: finish this
    let forms: Vec<PrintableLispForm> = read_delimited_vec(input, '}', curr_coord);
    let mut keys: Vec<PrintableLispForm> = Vec::new();
    let mut vals: Vec<PrintableLispForm> = Vec::new();
    let mut i = 0;

    while i < (forms.len()) {
        if let Some(key) = forms.get(i) {
            if let Some(val) = forms.get(i + 1) {
                keys.push(key.clone());
                vals.push(val.clone());
            }
        }
        i = i + 2;
    }
    (keys, vals)
}

// TODO: this should return a Result instead of a Option
fn read(input: &mut Peekable<Chars>, curr_coord: &Vec<u16>) -> Option<PrintableLispForm> {
    if let Some(c) = input.peek() {
        match c {
            '"' => Some(PrintableLispForm::String(read_string(input))),
            '(' => Some(PrintableLispForm::List {
                childs: read_delimited_vec(input, ')', curr_coord),
                style: PrintStyle::Unstyled,
                coord: curr_coord.clone(),
            }),
            '[' => Some(PrintableLispForm::Vector {
                childs: read_delimited_vec(input, ']', curr_coord),
                style: PrintStyle::Unstyled,
                coord: curr_coord.clone(),
            }),
            '#' => {
                input.next();
				let next_ch = input.peek().unwrap();
                if *next_ch == '{' {
					// it is a set
					Some(PrintableLispForm::Set {
						childs: read_delimited_vec(input, '}', curr_coord),
						style: PrintStyle::Unstyled,
						coord: curr_coord.clone(),
					})
				} else if *next_ch == '"' {
					// its a regex
					let exp = read_string(input);
					Some(PrintableLispForm::Regexp(exp))
				} else {
					// assume it is a tagged val
					let tag = read_atomic_token(input);                    
					let form = read(input, curr_coord).unwrap();
                    Some(PrintableLispForm::Tagged {
						tag: tag,
						form: Box::new(form),
						coord: curr_coord.clone()
					})
				}
                
            }
            '{' => {
                let (keys, vals) = read_map(input, curr_coord);
                Some(PrintableLispForm::Map {
                    keys: keys,
                    vals: vals,
                    style: PrintStyle::Unstyled,
                    coord: curr_coord.clone(),
                })
            }
            _ => Some(PrintableLispForm::Atomic(
                read_atomic_token(input),
                curr_coord.clone(),
            )),
        }
    } else {
        None
    }
}

pub fn read_str(input: &str) -> Option<PrintableLispForm> {
    let mut input_iter = input.chars().peekable();
    read(&mut input_iter, &Vec::new())
}

fn lisp_form_vec_to_str(v: Vec<PrintableLispForm>) -> String {
    v.iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

impl ToString for PrintableLispForm {
    fn to_string(&self) -> String {
        match self {
            PrintableLispForm::String(s) => format!("\"{}\"", s),
            PrintableLispForm::Regexp(exp) => format!("#\"{}\"", exp),
            PrintableLispForm::Atomic(s, _) => format!("{}", s),
            PrintableLispForm::List {
                childs,
                style: _,
                coord: _,
            } => {
                format!("({})", lisp_form_vec_to_str(childs.to_vec()))
            }
            PrintableLispForm::Vector {
                childs,
                style: _,
                coord: _,
            } => {
                format!("[{}]", lisp_form_vec_to_str(childs.to_vec()))
            }
            PrintableLispForm::Set {
                childs,
                style: _,
                coord: _,
            } => {
                format!("#{{{}}}", lisp_form_vec_to_str(childs.to_vec()))
            }
            PrintableLispForm::Map {
                keys,
                vals,
                style: _,
                coord: _,
            } => {
                let content = keys
                    .iter()
                    .zip(vals)
                    .map(|(k, v)| format!("{} {}", k.to_string(), v.to_string()))
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("{{{}}}", content)
            },
			PrintableLispForm::Tagged {
                tag,
                form,
				coord: _
            } => {                
                format!("#{}{}", tag, form.to_string())
            }
			
        }
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string_test() {
        let mut r_str = String::from("");
        if let Some(form) = read_str("\"this is a string\"") {
            if let PrintableLispForm::String(s) = form {
                r_str = s;
            }
        }
        assert_eq!(r_str, String::from("this is a string"));
    }

    #[test]
    fn read_atomic_token_test() {
        let mut r_str = String::from("");
        if let Some(form) = read_str("some_token") {
            if let PrintableLispForm::Atomic(s, _) = form {
                r_str = s;
            }
        }
        assert_eq!(r_str, String::from("some_token"));
    }

    #[test]
    fn read_delimited_vec_simple_test() {
        let mut r: Vec<PrintableLispForm> = Vec::new();
        if let Some(form) = read_str("[1 2 3 4]") {
            if let PrintableLispForm::Vector {
                childs,
                style: _,
                coord: _,
            } = form
            {
                r = childs;
            }
        }
        assert_eq!(r.len(), 4);
    }

    #[test]
    fn read_delimited_vec_nested_test() {
        let mut r: Vec<PrintableLispForm> = Vec::new();
        if let Some(form) = read_str("[#{1 something} 8 [2 3] (hello \"world\" 5)]") {
            if let PrintableLispForm::Vector {
                childs,
                style: _,
                coord: _,
            } = form
            {
                r = childs;
            }
        }
        assert_eq!(r.len(), 4);
    }

    #[test]
    fn read_str_map_test() {
        let mut rkeys: Vec<PrintableLispForm> = Vec::new();
        let mut rvals: Vec<PrintableLispForm> = Vec::new();
        if let Some(form) = read_str("{1 2 3 4}") {
            if let PrintableLispForm::Map {
                keys,
                vals,
                style: _,
                coord: _,
            } = form
            {
                rkeys = keys;
                rvals = vals;
            }
        }
        assert_eq!(rkeys.len(), 2);
        assert_eq!(rvals.len(), 2);
    }

    #[test]
    fn read_str_code_1_test() {
        let mut r: Vec<PrintableLispForm> = Vec::new();
        if let Some(form) =
            read_str("(defn factorial [n] (if (zero? n) 1 (* n (factorial (dec n)))))")
        {
            if let PrintableLispForm::List {
                childs,
                style: _,
                coord: _,
            } = form
            {
                r = childs;
            }
        }
        assert_eq!(r.len(), 4);
    }

    #[test]
    fn read_str_code_2_test() {
        let mut r: Vec<PrintableLispForm> = Vec::new();
        if let Some(form) = read_str("(let [a [1 2 3] b {:n/a 1, :c 2}] a)") {
            if let PrintableLispForm::List {
                childs,
                style: _,
                coord: _,
            } = form
            {
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

	#[test]
    fn tagged_1_test() {
        let input = "#atom[{1 2} 0x25176608]";

        if let Some(form) = read_str(input) {
            assert_eq!(form.to_string(), String::from(input));
        } else {
            assert!(false);
        }
    }

	#[test]
    fn regex_test() {
        let input = r#"(str/split something #".*")"#;

        if let Some(form) = read_str(input) {
			println!("{:?}",form);
            assert_eq!(form.to_string(), String::from(input));
        } else {
            assert!(false);
        }
    }
}
