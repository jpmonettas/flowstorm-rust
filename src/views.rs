use crate::lisp_pprinter;
use crate::lisp_pprinter::PrintToken;
use crate::lisp_reader;
use crate::lisp_reader::PrintableLispForm;
use crate::state::Form;
use crate::state::{Coord, DebuggerState, DebuggerTool, ExecTrace, Flow, FlowThread, FlowTool};
use crate::util_types::CallStackTreeNode;
use egui::{Align, Color32, Label, Layout, RichText, Sense, TextStyle, Ui};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct DebuggerApp {
    state_arc: Arc<Mutex<DebuggerState>>,
    pub ctx_chan_sender: Sender<egui::CtxRef>,
}

impl DebuggerApp {
    pub fn new(
        state_arc: Arc<Mutex<DebuggerState>>,
        ctx_chan_sender: Sender<egui::CtxRef>,
    ) -> Self {
        Self {
            state_arc,
            ctx_chan_sender,
        }
    }
}

fn hot_token_label(ui: &mut Ui, thread: &mut FlowThread, form: &Form, coord: &Coord, text: &str) {
    let mut rich_text = RichText::new(text);
    if thread.is_coord_hot(form.form_id, coord) {
        rich_text = rich_text.color(Color32::YELLOW);
        let curr_executing = thread
            .execution
            .is_current_coord_executing(form.form_id, coord);
        if curr_executing {
            rich_text = rich_text.color(Color32::GREEN);
        }

        let coord_traces = &thread.execution.traces_for_coord(form.form_id, coord);

        if coord_traces.len() > 1 {
            if !curr_executing {
                rich_text = rich_text.color(Color32::from_rgb(245, 126, 7));
            }
            let label = Label::new(rich_text).sense(Sense::click());
            let label_ctx_menu = |ui: &mut Ui| {
                for (trace_idx, t) in coord_traces {
                    if ui.button(&t.result).clicked() {
                        thread.jump_to(trace_idx);
                        ui.close_menu();
                    }
                }
            };

            if ui.add(label).context_menu(label_ctx_menu).clicked() {
                let (idx, _) = coord_traces[0];
                thread.jump_to(&idx);
            }
        } else {
            if ui
                .add(Label::new(rich_text).sense(Sense::click()))
                .clicked()
            {
                if let Some(e) = coord_traces.iter().next() {
                    let (trace_idx, _) = e;
                    thread.jump_to(trace_idx);
                }
            };
        }
    } else {
        ui.label(rich_text);
    }
}

fn flow_callstack_tree(
    ui: &mut Ui,
    flow_thread: &FlowThread,
    tree_pointer_mut: &Arc<Mutex<CallStackTreeNode>>,
) {
    let tree = tree_pointer_mut.lock().unwrap();
    let idx = tree.trace_idx;

    if let ExecTrace::FnCallTrace(fct) = &flow_thread.execution.traces[idx] {
        let fq_fn_name = format!("{}/{} ", &fct.fn_ns, &fct.fn_name);
        let fn_args = &fct.args_vec[1..&fct.args_vec.len() - 1];
        let fn_args_text = &fn_args[0..usize::min(80, fn_args.len())];
        let fn_call_text = format!("({} {})", fq_fn_name, fn_args_text);

        let ch = egui::CollapsingHeader::new(fn_call_text).id_source(idx);
        ch.show(ui, |ui| {
            for child in &tree.childs {
                flow_callstack_tree(ui, flow_thread, child);
            }
        });
    } else {
        panic!("call_stack_tree is pointing to a non FnCallTrace");
    }
}

fn flow_call_stack_block(ui: &mut Ui, flow_thread: &mut FlowThread) {
    if let Some(cst) = &flow_thread.call_stack_tree {
        let root_pointer = Arc::clone(&cst.root);
        flow_callstack_tree(ui, flow_thread, &root_pointer);
    }
}

fn flow_code_block(ui: &mut Ui, forms: Vec<&Form>, flow_thread: &mut FlowThread) {
    let initial_size = egui::vec2(
        ui.available_width(),
        ui.spacing().interact_size.y, // Assume there will be
    );

    let layout = Layout::left_to_right()
        .with_main_wrap(true)
        .with_cross_align(Align::BOTTOM);

    ui.allocate_ui_with_layout(initial_size, layout, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let row_height = (*ui.fonts())[TextStyle::Body].row_height();
        ui.set_row_height(row_height);

        for form in forms.iter() {
            if let ExecTrace::FnCallTrace(fct) = &flow_thread.execution.executing_trace() {
                if fct.form_id == form.form_id {
                    let fn_call_text = format!("({} {})", &fct.fn_name, &fct.args_vec);
                    let fn_call_text =
                        RichText::new(&fn_call_text[0..usize::min(80, fn_call_text.len())])
                            .color(Color32::GREEN);
                    ui.label(fn_call_text);
                    ui.allocate_exact_size(egui::vec2(0.0, row_height), Sense::hover()); // make sure we take up some height
                    ui.end_row();
                    ui.set_row_height(row_height);
                }
            }

            for t in &form.print_tokens {
                match t {
                    PrintToken::String(s) => {
                        ui.label(RichText::new(format!("\"{}\"", s)));
                    }
                    PrintToken::Regexp(exp) => {
                        ui.label(RichText::new(format!("#\"{}\"", exp)));
                    }
                    PrintToken::BlockOpen { val, coord } => {
                        hot_token_label(ui, flow_thread, form, coord, val);
                    }
                    PrintToken::BlockClose { val, coord } => {
                        hot_token_label(ui, flow_thread, form, coord, val);
                    }
                    PrintToken::Atomic { val, coord } => {
                        hot_token_label(ui, flow_thread, form, coord, val);
                    }
                    PrintToken::Space => {
                        ui.label(RichText::new(" "));
                    }
                    PrintToken::Newline => {
                        ui.allocate_exact_size(egui::vec2(0.0, row_height), Sense::hover()); // make sure we take up some height
                        ui.end_row();
                        ui.set_row_height(row_height);
                    }
                    PrintToken::PrintTokensVec(_) => {
                        panic!("all print tokens should be flatten at this stage")
                    }
                }
            }
            // Add some lines to separate rows
            ui.allocate_exact_size(egui::vec2(0.0, row_height * 3.0), Sense::hover());
            ui.end_row();
            ui.set_row_height(row_height);
        }
    });
}

fn flow_code_panel(ui: &mut Ui, forms: Vec<&Form>, flow_thread: &mut FlowThread) {
    egui::TopBottomPanel::top("flow_control_panel").show_inside(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Prev").clicked() {
                flow_thread.step_back();
            }

            ui.label(format!(
                "[{}/{}]",
                flow_thread.execution.curr_trace_idx,
                flow_thread.execution.traces.len()
            ));
            if ui.button("Next").clicked() {
                flow_thread.step_next();
            }
        });
    });
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            flow_code_block(ui, forms, flow_thread);
        });
    });
}

fn flow_call_stack_panel(ui: &mut Ui, flow_thread: &mut FlowThread) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            flow_call_stack_block(ui, flow_thread);
        });
    });
}

fn seq_collapsing_header(ui: &mut Ui, form: &PrintableLispForm) {
    match form {
        PrintableLispForm::List {
            childs,
            style: _,
            coord,
        }
        | PrintableLispForm::Vector {
            childs,
            style: _,
            coord,
        }
        | PrintableLispForm::Set {
            childs,
            style: _,
            coord,
        } => {
            // since the form is unstyled is going to print linear
            let linear_print =
                lisp_pprinter::print_tokens_to_str(&lisp_pprinter::lisp_form_print_tokens(form));
            let ch =
                egui::CollapsingHeader::new(&linear_print[0..usize::min(80, linear_print.len())])
                    .id_source(coord);
            ch.show(ui, |ui| {
                for c in childs {
                    result_form_tree(ui, c);
                }
            });
        }
        PrintableLispForm::Tagged { tag, form, coord } => {
            // since the form is unstyled is going to print linear
            let tagged_body =
                lisp_pprinter::print_tokens_to_str(&lisp_pprinter::lisp_form_print_tokens(form));
            let linear_print = format!(
                "#{}{}",
                tag,
                &tagged_body[0..usize::min(80, tagged_body.len())]
            );

            let ch = egui::CollapsingHeader::new(linear_print).id_source(coord);
            ch.show(ui, |ui| {
                result_form_tree(ui, form);
            });
        }
        _ => {
            panic!("seq_collapsing_header called with a non seq form");
        }
    }
}

fn result_form_tree(ui: &mut Ui, form: &PrintableLispForm) {
    match form {
        PrintableLispForm::Atomic(s, _) => {
            ui.label(s);
        }
        PrintableLispForm::String(s) => {
            ui.label(format!("\"{}\"", s));
        }
        PrintableLispForm::Map {
            keys,
            vals,
            style: _,
            coord,
        } => {
            // TODO: we can't print a map linear since the printer doesn't support it
            egui::CollapsingHeader::new("{...}")
                .id_source(coord)
                .show(ui, |ui| {
                    for (k, v) in keys.iter().zip(vals) {
                        let linear_key_print = lisp_pprinter::print_tokens_to_str(
                            &lisp_pprinter::lisp_form_print_tokens(k),
                        );
                        ui.horizontal_wrapped(|ui| {
                            ui.label(linear_key_print);
                            result_form_tree(ui, v);
                        });
                    }
                });
        }
        _ => seq_collapsing_header(ui, form),
    }
}

fn flow_result(ui: &mut Ui, flow_thread: &FlowThread) {
    if let Some(form) = &flow_thread.value_inspector {
        result_form_tree(ui, form);
    }
}

fn flow_locals(ui: &mut Ui, flow_thread: &mut FlowThread) {
    egui::Grid::new("locals").show(ui, |ui| {
        ui.set_min_height(ui.available_height() / 3.0);
        for (symb, val) in flow_thread.current_locals() {
            ui.label(symb);
            ui.label(val);
            ui.end_row();
        }
    });
}

fn flow_thread(
    ui: &mut Ui,
    _ctx: &egui::CtxRef,
    forms: Vec<&Form>,
    selected_flow_thread: &mut FlowThread,
) {
    ui.group(|ui| {
        egui::SidePanel::right("results_and_locals_panel")
            .resizable(true)
            .default_width(ui.available_width() / 2.0)
            .max_width(ui.available_width() - (ui.available_width() / 3.0))
            .show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    egui::ScrollArea::both()
                        .max_height(ui.available_height() / 2.0)
                        .show(ui, |ui| {
                            flow_result(ui, selected_flow_thread);
                        });

                    if selected_flow_thread.selected_flow_tool == FlowTool::Code {
                        ui.group(|ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.set_height(ui.available_height());
                                flow_locals(ui, selected_flow_thread);
                            });
                        });
                    }
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::TopBottomPanel::bottom("left_side_tabs_panel").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .selectable_label(
                            selected_flow_thread.selected_flow_tool == FlowTool::Code,
                            "Code",
                        )
                        .clicked()
                    {
                        selected_flow_thread.selected_flow_tool = FlowTool::Code;
                    } else if ui
                        .selectable_label(
                            selected_flow_thread.selected_flow_tool == FlowTool::CallStack,
                            "Call stack",
                        )
                        .clicked()
                    {
                        selected_flow_thread.selected_flow_tool = FlowTool::CallStack;
                    }
                });
            });

            match selected_flow_thread.selected_flow_tool {
                FlowTool::Code => {
                    flow_code_panel(ui, forms, selected_flow_thread);
                }
                FlowTool::CallStack => {
                    flow_call_stack_panel(ui, selected_flow_thread);
                }
            }
        });
    });
}

fn flow_threads(ui: &mut Ui, ctx: &egui::CtxRef, selected_flow: &mut Flow) {
    if let Some(selected_thread_id) = selected_flow.selected_thread_id {
        egui::TopBottomPanel::top("thread_selection_panel").show_inside(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for thread_id in selected_flow.thread_ids() {
                    if ui
                        .selectable_label(
                            selected_thread_id == thread_id,
                            format!("thread-{}", thread_id),
                        )
                        .clicked()
                    {
                        selected_flow.selected_thread_id = Some(thread_id);
                    }
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // HACKY, this shouldn't be here, but you know, borrow checker
            let mut selected_thread_forms = Vec::new();
            {
                for form_id in selected_flow
                    .threads
                    .get(&selected_thread_id)
                    .unwrap()
                    .hot_coords
                    .keys()
                {
                    selected_thread_forms.push(selected_flow.forms.get(form_id).unwrap());
                }
            }

            flow_thread(
                ui,
                ctx,
                selected_thread_forms,
                selected_flow.threads.get_mut(&selected_thread_id).unwrap(),
            );
        });
    }
}

fn flows_tool(ui: &mut Ui, ctx: &egui::CtxRef, state: &mut DebuggerState) {
    if state.flows.is_empty() {
        ui.heading("No flows yet");
    } else {
        egui::TopBottomPanel::top("flows_selection_panel").show_inside(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for flow_id in state.flows_ids() {
                    if ui
                        .selectable_label(
                            state.selected_flow().unwrap().flow_id == flow_id,
                            format!("Flow-{}", flow_id),
                        )
                        .clicked()
                    {
                        state.select_flow(flow_id);
                    }
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(ref mut selected_flow) = state.selected_flow_mut() {
                flow_threads(ui, ctx, selected_flow);
            }
        });
    }
}

fn refs_tool(ui: &mut Ui, _state: &mut DebuggerState) {
    ui.heading("REFS");
}

fn taps_tool(ui: &mut Ui, _state: &mut DebuggerState) {
    ui.heading("TAPS");
}

fn timeline_tool(ui: &mut Ui, _state: &mut DebuggerState) {
    ui.heading("TIMELINE");
}

impl epi::App for DebuggerApp {
    fn name(&self) -> &str {
        "Flowstorm debugger"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let _r = self.ctx_chan_sender.send(egui::CtxRef::clone(ctx));

        let mut fonts = egui::FontDefinitions::default();

        fonts
            .family_and_size
            .insert(TextStyle::Body, (egui::FontFamily::Proportional, 15.0));

        fonts
            .family_and_size
            .insert(TextStyle::Button, (egui::FontFamily::Proportional, 15.0));

        ctx.set_fonts(fonts);
        // Enable for debugging widgets layout
        // ctx.set_debug_on_hover(true);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        // This is not optimal since we are keeping the lock for the entire frame
        let mut state = self.state_arc.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("tool_selection_panel").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                    ui.separator();

                    if ui
                        .selectable_label(state.selected_tool == DebuggerTool::Flows, "Flows")
                        .clicked()
                    {
                        state.selected_tool = DebuggerTool::Flows;
                    } else if ui
                        .selectable_label(state.selected_tool == DebuggerTool::Refs, "Refs")
                        .clicked()
                    {
                        state.selected_tool = DebuggerTool::Refs;
                    } else if ui
                        .selectable_label(state.selected_tool == DebuggerTool::Taps, "Taps")
                        .clicked()
                    {
                        state.selected_tool = DebuggerTool::Taps;
                    } else if ui
                        .selectable_label(state.selected_tool == DebuggerTool::Timeline, "Timeline")
                        .clicked()
                    {
                        state.selected_tool = DebuggerTool::Timeline;
                    }
                });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| match state.selected_tool {
                DebuggerTool::Flows => flows_tool(ui, ctx, &mut state),
                DebuggerTool::Refs => refs_tool(ui, &mut state),
                DebuggerTool::Taps => taps_tool(ui, &mut state),
                DebuggerTool::Timeline => timeline_tool(ui, &mut state),
            });

            egui::TopBottomPanel::bottom("bottom_panel").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if let Some(cpu_usage) = frame.info().cpu_usage {
                        ui.label(format!(
                            "Frame duration: {:.2}ms ~ {:.0}fps",
                            cpu_usage * 1000.0,
                            1.0 / cpu_usage
                        ));
                        ui.separator();
                        ui.label(format!("Trace count: {}", state.total_trace_count));
                    }

                    ui.separator();
                });
            });
        });
    }
}
