/***********************************************************

- can we improve all with visitors pattern?

***********************************************************/

use crate::lisp_reader::PrintStyle;
use crate::lisp_reader::PrintableLispForm;

#[derive(Debug, Clone, PartialEq)]
pub enum PrintToken {
    String(String),
    BlockOpen { val: String, coord: Vec<u16> },
    BlockClose { val: String, coord: Vec<u16> },
    Atomic { val: String, coord: Vec<u16> },
    Space,
    Newline,
    PrintTokensVec(Vec<PrintToken>),
}

const INDENT_WIDTH: usize = 2;

////////////////////
// Some utilities //
////////////////////

fn indented_nl(width: usize) -> Vec<PrintToken> {
    let mut v: Vec<PrintToken> = Vec::new();
    v.push(PrintToken::Newline);
    for _i in 0..width {
        v.push(PrintToken::Space)
    }
    v
}

// TODO: this can be replaced by iterator intersperse
fn join(pt: PrintToken, tokens: Vec<PrintToken>) -> Vec<PrintToken> {
    let mut r: Vec<PrintToken> = Vec::new();
    for t in tokens.iter().cloned() {
        r.push(t);
        r.push(pt.clone());
    }
    r.pop();
    r
}

fn join_coll(v: Vec<PrintToken>, tokens: Vec<PrintToken>) -> Vec<PrintToken> {
    let mut r: Vec<PrintToken> = Vec::new();
    for t in tokens.iter().cloned() {
        r.push(t);
        for vt in v.iter().cloned() {
            r.push(vt);
        }
    }
    for _i in 0..v.len() {
        r.pop();
    }

    r
}

////////////////////
// Implementation //
////////////////////

fn n_symbol_print(
    childs: &Vec<PrintableLispForm>,
    n: usize,
    left: usize,
) -> (Vec<PrintToken>, usize) {
    let elem = childs.as_slice().get(n).unwrap();
    let toks = lisp_form_print_tokens_aux(elem, left);
    let toks_width = tokens_width(&flatten_print_tokens(&toks));
    (toks, toks_width)
}

fn flatten_print_tokens(tokens: &Vec<PrintToken>) -> Vec<PrintToken> {
    let mut r: Vec<PrintToken> = Vec::new();
    for pt in tokens.iter().cloned() {
        match &pt {
            PrintToken::String(_) => {
                r.push(pt);
            }
            PrintToken::BlockOpen { val: _, coord: _ } => {
                r.push(pt);
            }
            PrintToken::BlockClose { val: _, coord: _ } => {
                r.push(pt);
            }
            PrintToken::Atomic { val: _, coord: _ } => {
                r.push(pt);
            }
            PrintToken::Space => {
                r.push(pt);
            }
            PrintToken::Newline => {
                r.push(pt);
            }
            PrintToken::PrintTokensVec(v) => {
                for t in flatten_print_tokens(&v).iter().cloned() {
                    r.push(t);
                }
            }
        }
    }
    r
}

fn tokens_width(tokens: &Vec<PrintToken>) -> usize {
    tokens
        .as_slice()
        .rsplit(|pt| {
            if let PrintToken::Newline = pt {
                true
            } else {
                false
            }
        })
        .map(|pt_line| {
            pt_line
                .iter()
                .map(|pt| match pt {
                    PrintToken::String(s) => s.len(),
                    PrintToken::BlockOpen {
                        ref val,
                        coord: _coord,
                    } => val.len(),
                    PrintToken::BlockClose {
                        ref val,
                        coord: _coord,
                    } => val.len(),
                    PrintToken::Atomic {
                        ref val,
                        coord: _coord,
                    } => val.len(),
                    PrintToken::Space => 1,
                    PrintToken::Newline => 1,
                    PrintToken::PrintTokensVec(_v) => {
                        panic!("tokens_width only work with a flat Vec<PrintToken")
                    }
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn lisp_form_seq_print_tokens(pform: &PrintableLispForm, left: usize) -> Vec<PrintToken> {
    match pform {
        PrintableLispForm::List {
            childs,
            style,
            coord: _coord,
        }
        | PrintableLispForm::Vector {
            childs,
            style,
            coord: _coord,
        }
        | PrintableLispForm::Set {
            childs,
            style,
            coord: _coord,
        } => {
            if childs.is_empty() {
                return Vec::new();
            } else {
                match style {
                    PrintStyle::Unstyled | PrintStyle::Linear => {
                        return join(
                            PrintToken::Space,
                            childs
                                .iter()
                                .map(|lf| {
                                    PrintToken::PrintTokensVec(lisp_form_print_tokens_aux(lf, left))
                                })
                                .collect::<Vec<PrintToken>>(),
                        )
                    }

                    PrintStyle::Standard => {
                        // length of the first thing on the List, Vector, Set
                        let (_, flen) = n_symbol_print(childs, 0, left);
                        childs
                            .iter()
                            .enumerate()
                            .map(|(i, pf)| match i {
                                0 => {
                                    PrintToken::PrintTokensVec(lisp_form_print_tokens_aux(pf, left))
                                }
                                1 => {
                                    let mut r: Vec<PrintToken> = Vec::new();
                                    let mut v = lisp_form_print_tokens_aux(pf, left + flen + 1);
                                    r.push(PrintToken::Space);
                                    r.append(&mut v);
                                    PrintToken::PrintTokensVec(r)
                                }
                                _ => {
                                    let mut r: Vec<PrintToken> = Vec::new();
                                    let mut iln = indented_nl(left + flen + 2);
                                    let mut v = lisp_form_print_tokens_aux(pf, left + flen + 2);
                                    r.append(&mut iln);
                                    r.append(&mut v);
                                    PrintToken::PrintTokensVec(r)
                                }
                            })
                            .collect::<Vec<PrintToken>>()
                    }

                    PrintStyle::Defn => {
                        let (a_toks, a_len) = n_symbol_print(childs, 0, left);
                        let (b_toks, b_len) = n_symbol_print(childs, 1, left + a_len + 1);
                        let (c_toks, _) = n_symbol_print(childs, 2, left + a_len + b_len + 1);
                        let rest = childs.iter().skip(3);
                        return vec![
                            PrintToken::PrintTokensVec(a_toks),
                            PrintToken::Space,
                            PrintToken::PrintTokensVec(b_toks),
                            PrintToken::Space,
                            PrintToken::PrintTokensVec(c_toks),
                            PrintToken::Space,
                            PrintToken::PrintTokensVec(
                                rest.map(|lf| {
                                    // TODO: we can factor this out since it is also used in Standard
                                    let mut r: Vec<PrintToken> = Vec::new();
                                    let mut iln = indented_nl(left + INDENT_WIDTH);
                                    let mut v = lisp_form_print_tokens_aux(lf, left + INDENT_WIDTH);
                                    r.append(&mut iln);
                                    r.append(&mut v);
                                    PrintToken::PrintTokensVec(r)
                                })
                                .collect::<Vec<PrintToken>>(),
                            ),
                        ];
                    }

                    PrintStyle::PairsBlock => {
                        let parts = childs
                            .chunks(2)
                            .map(|part| {
                                let lf1 = &part[0];
                                let lf2 = &part[1];
                                let mut lf1c = lf1.clone();
                                match lf1c {
                                    PrintableLispForm::List {
                                        childs: ref _childs,
                                        ref mut style,
                                        coord: ref _coord,
                                    }
                                    | PrintableLispForm::Vector {
                                        childs: ref _childs,
                                        ref mut style,
                                        coord: ref _coord,
                                    }
                                    | PrintableLispForm::Set {
                                        childs: ref _childs,
                                        ref mut style,
                                        coord: ref _coord,
                                    } => {
                                        *style = PrintStyle::Linear;
                                    }
                                    _ => (),
                                };

                                let lf1_toks = lisp_form_print_tokens_aux(&lf1c, left);
                                let lf1_len = tokens_width(&flatten_print_tokens(&lf1_toks));
                                let lf2_toks = lisp_form_print_tokens_aux(&lf2, left + 1 + lf1_len);

                                let mut r: Vec<PrintToken> = Vec::new();
                                r.push(PrintToken::PrintTokensVec(lf1_toks));
                                r.push(PrintToken::Space);
                                r.push(PrintToken::PrintTokensVec(lf2_toks));
                                PrintToken::PrintTokensVec(r)
                            })
                            .collect::<Vec<PrintToken>>();

                        return join_coll(indented_nl(left + INDENT_WIDTH), parts);
                    }

                    PrintStyle::Binding => {
                        let (symb_toks, symb_len) = n_symbol_print(childs, 0, left);
                        let mut bvec_form = childs.iter().skip(1).next().unwrap().clone();

                        match bvec_form {
                            PrintableLispForm::Vector {
                                childs: ref _childs,
                                ref mut style,
                                coord: ref _coord,
                            } => {
                                *style = PrintStyle::PairsBlock;
                            }
                            _ => (),
                        };
                        let bvec_toks = lisp_form_print_tokens_aux(&bvec_form, left + symb_len + 2);

                        let rest = childs.iter().skip(2);

                        vec![
                            PrintToken::PrintTokensVec(symb_toks),
                            PrintToken::Space,
                            PrintToken::PrintTokensVec(bvec_toks),
                            PrintToken::Space,
                            PrintToken::PrintTokensVec(
                                rest.map(|lf| {
                                    // TODO: we can factor this out since it is also used in Standard
                                    let mut r: Vec<PrintToken> = Vec::new();
                                    let mut iln = indented_nl(left + INDENT_WIDTH);
                                    let mut v = lisp_form_print_tokens_aux(lf, left + INDENT_WIDTH);
                                    r.append(&mut iln);
                                    r.append(&mut v);
                                    PrintToken::PrintTokensVec(r)
                                })
                                .collect::<Vec<PrintToken>>(),
                            ),
                        ]
                    }
                }
            }
        }
        _ => panic!("lisp_form_seq_print_tokens is only for List, Vector or Set"),
    }
}

fn map_body_print_tokens(pform: &PrintableLispForm, left: usize) -> Vec<PrintToken> {
    if let PrintableLispForm::Map {
        keys,
        vals,
        style: _,
        coord,
    } = pform
    {
        // Kind of hacky but if we interleave keys and vals we can print the map body as a list in PairsBlock style
        return lisp_form_seq_print_tokens(
            &PrintableLispForm::List {
                childs: keys
                    .iter()
                    .zip(vals)
                    .flat_map(|(x, y)| {
                        let v: Vec<PrintableLispForm> = vec![x.clone(), y.clone()];
                        v
                    })
                    .collect::<Vec<PrintableLispForm>>(),
                style: PrintStyle::PairsBlock,
                coord: coord.to_vec(),
            },
            left,
        );
    } else {
        return Vec::new();
    }
}

fn lisp_form_print_tokens_aux(pform: &PrintableLispForm, left: usize) -> Vec<PrintToken> {
    match pform {
        PrintableLispForm::List {
            childs: _,
            style: _,
            coord,
        } => vec![
            PrintToken::BlockOpen {
                val: "(".to_string(),
                coord: coord.to_vec(),
            },
            PrintToken::PrintTokensVec(lisp_form_seq_print_tokens(pform, left)),
            PrintToken::BlockClose {
                val: ")".to_string(),
                coord: coord.to_vec(),
            },
        ],

        PrintableLispForm::Vector {
            childs: _,
            style: _,
            coord,
        } => vec![
            PrintToken::BlockOpen {
                val: "[".to_string(),
                coord: coord.to_vec(),
            },
            PrintToken::PrintTokensVec(lisp_form_seq_print_tokens(pform, left)),
            PrintToken::BlockClose {
                val: "]".to_string(),
                coord: coord.to_vec(),
            },
        ],

        PrintableLispForm::Set {
            childs: _,
            style: _,
            coord,
        } => vec![
            PrintToken::BlockOpen {
                val: "#{".to_string(),
                coord: coord.to_vec(),
            },
            PrintToken::PrintTokensVec(lisp_form_seq_print_tokens(pform, left)),
            PrintToken::BlockClose {
                val: "}".to_string(),
                coord: coord.to_vec(),
            },
        ],

        PrintableLispForm::Map {
            keys: _,
            vals: _,
            style: _,
            coord,
        } => vec![
            PrintToken::BlockOpen {
                val: "{".to_string(),
                coord: coord.to_vec(),
            },
            PrintToken::PrintTokensVec(map_body_print_tokens(pform, left)),
            PrintToken::BlockClose {
                val: "}".to_string(),
                coord: coord.to_vec(),
            },
        ],

        PrintableLispForm::String(s) => vec![PrintToken::String(s.to_string())],

        PrintableLispForm::Atomic(s, coord) => vec![PrintToken::Atomic {
            val: s.to_string(),
            coord: coord.to_vec(),
        }],
    }
}

fn lisp_form_print_tokens(pform: &PrintableLispForm) -> Vec<PrintToken> {
    flatten_print_tokens(&lisp_form_print_tokens_aux(pform, 0))
}

fn symb_style_lisp_form_deep(pform: &mut PrintableLispForm) {
    match pform {
        PrintableLispForm::List {
            childs,
            ref mut style,
            coord: _,
        } => {
            if let Some(e) = childs.as_slice().first() {
                if let PrintableLispForm::Atomic(s, _) = e {
                    *style = match s.as_ref() {
                        "defn" => PrintStyle::Defn,
                        "let" => PrintStyle::Binding,
                        "binding" => PrintStyle::Binding,
                        _ => PrintStyle::Unstyled,
                    }
                }
            }
            for c in childs {
                symb_style_lisp_form_deep(c)
            }
        }
        PrintableLispForm::Vector {
            childs,
            style: _,
            coord: _,
        } => {
            for c in childs {
                symb_style_lisp_form_deep(c)
            }
        }
        PrintableLispForm::Set {
            childs,
            style: _,
            coord: _,
        } => {
            for c in childs {
                symb_style_lisp_form_deep(c)
            }
        }
        PrintableLispForm::Map {
            keys,
            vals,
            style: _,
            coord: _,
        } => {
            for c in keys {
                symb_style_lisp_form_deep(c)
            }
            for c in vals {
                symb_style_lisp_form_deep(c)
            }
        }
        PrintableLispForm::String(_) => (),
        PrintableLispForm::Atomic(_, _) => (),
    }
}

fn standard_style_next_unstyled_childs(childs: &mut Vec<PrintableLispForm>) -> bool {
    childs
        .iter_mut()
        .map(|c| standard_style_next_unstyled(c))
        .fold(false, |acc, n| acc || n)
}

fn standard_style_next_unstyled(pform: &mut PrintableLispForm) -> bool {
    match pform {
        PrintableLispForm::List {
            ref mut childs,
            ref mut style,
            coord: _,
        } => {
            if let PrintStyle::Unstyled = style {
                *style = PrintStyle::Standard;
                return true;
            } else {
                return standard_style_next_unstyled_childs(childs);
            }
        }
        PrintableLispForm::Vector {
            ref mut childs,
            style: _,
            coord: _,
        } => standard_style_next_unstyled_childs(childs),
        PrintableLispForm::Set {
            ref mut childs,
            style: _,
            coord: _,
        } => standard_style_next_unstyled_childs(childs),
        PrintableLispForm::Map {
            ref mut keys,
            ref mut vals,
            style: _,
            coord: _,
        } => standard_style_next_unstyled_childs(keys) || standard_style_next_unstyled_childs(vals),
        PrintableLispForm::String(_) => false,
        PrintableLispForm::Atomic(_, _) => false,
    }
}

pub fn style_lisp_form(pform: &mut PrintableLispForm, width: usize) -> Vec<PrintToken> {
    symb_style_lisp_form_deep(pform);
    loop {
        let curr_toks = lisp_form_print_tokens(pform);
        let curr_width = tokens_width(&curr_toks);

        if curr_width < width {
            return curr_toks;
        } else {
            let shorter = standard_style_next_unstyled(pform);
            if !shorter {
                return curr_toks;
            }
        }
    }
}

// For tests and debugging
#[allow(dead_code)]
pub fn print_tokens_to_str(tokens: &Vec<PrintToken>) -> String {
    let mut r = String::new();

    for t in tokens {
        match t {
            PrintToken::String(s) => r.push_str(&format!("\"{}\"", s)),
            PrintToken::BlockOpen { val, coord: _ } => r.push_str(&val),
            PrintToken::BlockClose { val, coord: _ } => r.push_str(&val),
            PrintToken::Atomic { val, coord: _ } => r.push_str(&val),
            PrintToken::Space => r.push_str(" "),
            PrintToken::Newline => r.push_str("\n"),
            PrintToken::PrintTokensVec(_) => {
                panic!("all print tokens should be flatten at this stage")
            }
        }
    }
    r
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lisp_reader;

    #[test]
    fn indented_nl_test() {
        let toks = indented_nl(3);
        assert_eq!(
            toks,
            vec![
                PrintToken::Newline,
                PrintToken::Space,
                PrintToken::Space,
                PrintToken::Space
            ]
        );
    }

    #[test]
    fn join_test() {
        assert_eq!(
            join(
                PrintToken::Space,
                vec![
                    PrintToken::String("a".to_string()),
                    PrintToken::String("b".to_string())
                ]
            ),
            vec![
                PrintToken::String("a".to_string()),
                PrintToken::Space,
                PrintToken::String("b".to_string())
            ]
        )
    }

    #[test]
    fn join_coll_test() {
        assert_eq!(
            join_coll(
                vec![PrintToken::Space, PrintToken::Space],
                vec![
                    PrintToken::String("a".to_string()),
                    PrintToken::String("b".to_string())
                ]
            ),
            vec![
                PrintToken::String("a".to_string()),
                PrintToken::Space,
                PrintToken::Space,
                PrintToken::String("b".to_string())
            ]
        )
    }

    #[test]
    fn lisp_form_print_tokens_test() {
        let form1 = lisp_reader::read_str("(\"a\", \"b\")").unwrap();
        let form1_toks = lisp_form_print_tokens(&form1);
        assert_eq!(
            form1_toks,
            vec![
                PrintToken::BlockOpen {
                    val: "(".to_string(),
                    coord: vec![]
                },
                PrintToken::String("a".to_string()),
                PrintToken::Space,
                PrintToken::String("b".to_string()),
                PrintToken::BlockClose {
                    val: ")".to_string(),
                    coord: vec![]
                }
            ]
        );
    }

    #[test]
    fn style_lisp_form_test() {
        let mut form1 = lisp_reader::read_str(
            "(defn factorial [n] (if (zero? n) 1 (* n (factorial (dec n)))))",
        )
        .unwrap();
        let form1_toks = style_lisp_form(&mut form1, 40);
        println!("{:?}", form1_toks);

        assert_eq!(
            print_tokens_to_str(&form1_toks),
            "(defn factorial [n] \n  (if (zero? n)\n      1\n      (* n (factorial (dec n)))))"
        );

        // let mut form2 = read_str("(defn styled-ast-tokens-seq [{:keys [ptype childs-vec]} left] (cond (or (= :linear ptype) (nil? ptype)) (join :space (mapv (fn [x] (styled-ast-tokens x left)) childs-vec)) (= :standard-style ptype) (let [flen (count (first (styled-ast-tokens (first childs-vec) left)))] (->> childs-vec (map-indexed (fn [i s] (case i 0 (styled-ast-tokens s left) 1 [space-sep (styled-ast-tokens s (+ left flen 1))] [(indented-nl (+ left flen 2)) (styled-ast-tokens s (+ left flen 2))]))) (into []))) (= :defn-style ptype) (let [[a b c & r] childs-vec a-toks (styled-ast-tokens a left) a-len (count (first a-toks)) b-toks (styled-ast-tokens b (+ left a-len 1)) b-len (count (first b-toks)) c-toks (styled-ast-tokens c (+ left a-len b-len 1))] [a-toks :space b-toks :space c-toks :space (mapv (fn [x] (into (indented-nl (+ left indent-width)) (styled-ast-tokens x (+ left indent-width)))) r)]) (= :pairs-block-style ptype) (->> (partition 2 childs-vec) (map (fn [[a b]] (let [a-toks (styled-ast-tokens (assoc a :ptype :linear) left) a-str (first a-toks)] [a-toks space-sep (styled-ast-tokens b (+ left (count a-str) 3))]))) (join-coll (indented-nl (+ left indent-width)))) (= :binding-style ptype) (let [[symb bvec & r] childs-vec symb-toks (styled-ast-tokens symb left) symb-str (first symb-toks) bvec-toks (styled-ast-tokens (assoc bvec :ptype :pairs-block-style) (+ left (count symb-str) 1))] [symb-toks :space bvec-toks :space (mapv (fn [x] (into (indented-nl (+ left indent-width)) (styled-ast-tokens x (+ left indent-width)))) r)])))").unwrap();
        // let form2_toks = style_lisp_form(&mut form2, 80);
        // let form2_out = print_tokens_to_str(&form2_toks);
        // println!("{:?}", form2_out);
        // println!("{}", form2_out);

        // assert_eq!(form2_out,
        // 		   "");

        // println!("{:?}", print_tokens_to_str(form1_toks));
        // assert!(false);
    }
}
