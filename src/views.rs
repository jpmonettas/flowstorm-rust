use crate::lisp_pprinter::PrintToken;
use crate::lisp_reader;
use crate::lisp_pprinter;
use crate::lisp_reader::PrintableLispForm;
use crate::state::Form;
use crate::state::{DebuggerState, Flow, FlowExecution, ExecTrace, ExprTrace};
use egui::{Align, Color32, Label, Layout, RichText, Sense, TextStyle, Ui};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
enum DebuggerTool {
    Flows,
    Refs,
    Taps,
    Timeline,
}

pub struct DebuggerApp {
    state_arc: Arc<Mutex<DebuggerState>>,
    pub ctx_chan_sender: Sender<egui::CtxRef>,
    selected_tool: DebuggerTool,
}

impl DebuggerApp {
    pub fn new(
        state_arc: Arc<Mutex<DebuggerState>>,
        ctx_chan_sender: Sender<egui::CtxRef>,
    ) -> Self {
        Self {
            state_arc,
            ctx_chan_sender,
            selected_tool: DebuggerTool::Flows,
        }
    }
}

fn hot_token_label(
    ui: &mut Ui,
    execution: &mut FlowExecution,
    form: &Form,
    coord: &[u16],
    text: &str,
) {
    let mut rich_text = RichText::new(text);
    if form.is_coord_hot(coord) {
        rich_text = rich_text.color(Color32::YELLOW);
		let curr_executing = execution.is_current_coord_executing(coord);
        if curr_executing {
            rich_text = rich_text.color(Color32::GREEN);
        }

        let coord_traces = &execution.traces_for_coord(coord);

        if coord_traces.len() > 1 {
			if !curr_executing {
				rich_text = rich_text.color(Color32::from_rgb(245, 126, 7));
			}            
            let label = Label::new(rich_text).sense(Sense::click());
			let label_ctx_menu = |ui: &mut Ui| { 
				for (trace_idx, t) in coord_traces {                    
                    if ui.button(&t.result)                        
						.clicked() {
							execution.jump_to(trace_idx);
							ui.close_menu();
						}
                }
			};
			
			if ui.add(label)
				.context_menu(label_ctx_menu) 
				.clicked() {
					let (idx, _) = coord_traces[0];
					execution.jump_to(&idx);
				}
        } else {
            if ui
                .add(Label::new(rich_text).sense(Sense::click()))
                .clicked()
            {
                if let Some(e) = coord_traces.iter().next() {
                    let (trace_idx, _) = e;
                    execution.jump_to(trace_idx);
                }
            };
        }
    } else {
        ui.label(rich_text);
    }
}

fn flow_code_block(ui: &mut Ui, flow: &mut Flow) {
    
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

        let forms = &flow.forms;

        for form in forms.values() {

			if let ExecTrace::FnCallTrace(fct) = &flow.execution.executing_trace() {
				let fn_call_text = RichText::new(format!("({} {})", fct.fn_name, fct.args_vec)).color(Color32::GREEN);
				ui.label(fn_call_text);
				ui.allocate_exact_size(egui::vec2(0.0, row_height), Sense::hover()); // make sure we take up some height
                ui.end_row();
                ui.set_row_height(row_height);
			}
			
            for t in form.print_tokens() {
                match t {
                    PrintToken::String(s) => {
                        ui.label(RichText::new(format!("\"{}\"",s)));
                    }
                    PrintToken::BlockOpen { val, coord } => {
                        hot_token_label(ui, &mut flow.execution, form, coord, val);
                    }
                    PrintToken::BlockClose { val, coord } => {
                        hot_token_label(ui, &mut flow.execution, form, coord, val);
                    }
                    PrintToken::Atomic { val, coord } => {
                        hot_token_label(ui, &mut flow.execution, form, coord, val);
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
            // Add a new line to separate forms
            ui.allocate_exact_size(egui::vec2(0.0, row_height), Sense::hover());
            ui.end_row();
            ui.set_row_height(row_height);
        }
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
            let linear_print = lisp_pprinter::print_tokens_to_str(&lisp_pprinter::lisp_form_print_tokens(form));
			let ch = egui::CollapsingHeader::new(linear_print).id_source(coord);
            ch.show(ui, |ui| {
                for c in childs {
                    result_form_tree(ui, c);
                }
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
            ui.label(format!("\"{}\"",s));
        }
        PrintableLispForm::Map {
            keys,
            vals,
            style: _,
            coord,
        } => {
            // TODO: we can't print a map linear since the printer doesn't support it
            egui::CollapsingHeader::new("{...}").id_source(coord).show(ui, |ui| {
				for (k, v) in keys.iter().zip(vals) {
					let linear_key_print = lisp_pprinter::print_tokens_to_str(&lisp_pprinter::lisp_form_print_tokens(k));
					ui.horizontal_wrapped(|ui| {
						ui.label(linear_key_print);
						result_form_tree(ui, v);    
					});					
				}
                
            });
        },
		_ => { seq_collapsing_header(ui, form) }
    }
}

fn flow_result(ui: &mut Ui, flow: &Flow) {
	if let ExecTrace::ExprTrace(et) = flow.execution.executing_trace() {
		let result_str = &et.result;
		if let Some(result_pf) = lisp_reader::read_str(result_str) { 
			result_form_tree(ui, &result_pf);
		} else { // If we can't parse the result, show a label with the string
			ui.label(result_str);
		}            
	}	
}

fn flow_locals(ui: &mut Ui, flow: &mut Flow) {
    egui::Grid::new("locals").show(ui, |ui| {
		ui.set_min_height(ui.available_height()/3.0);
        for (symb, val) in flow.current_locals() {
            ui.label(symb);
            ui.label(val);
            ui.end_row();
        }
    });    
}

fn flows_tool(ui: &mut Ui, ctx: &egui::CtxRef, state: &mut DebuggerState) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(selected_flow_id) = state.selected_flow_id {
            // Flow selector
            egui::TopBottomPanel::top("flows_selection_panel").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for flow in state.flows.values() {
                        if ui
                            .selectable_label(
                                selected_flow_id == flow.flow_id,
                                format!("{}", flow.flow_id),
                            )
                            .clicked()
                        {
                            state.selected_flow_id = Some(flow.flow_id);
                        }
                    }
                });
            });

			if let Some(ref mut selected_flow) = state.selected_flow_mut() {
				// Flow controls            
				ui.group(|ui| {
					{
						ui.horizontal_wrapped(|ui| {
							if ui.button("Prev").clicked() {
								selected_flow.execution.step_back();
							}

							ui.label(format!(
								"[{}/{}]",
								selected_flow.execution.curr_trace_idx,
								selected_flow.execution.traces.len()
							));
							if ui.button("Next").clicked() {
								selected_flow.execution.step_next();
							}
						});
					}
				});

				ui.group(|ui| {
					let available_height_for_right = ui.available_height();
					egui::SidePanel::right("results_and_locals_panel")
						.resizable(true)
						.default_width(ui.available_width()/2.0)
						.max_width(ui.available_width() - (ui.available_width()/3.0))
						.show_inside(ui, |ui| {
							ui.vertical(|ui| {
								egui::ScrollArea::both()
									.max_height(ui.available_height()/2.0)                                    
									.show(ui, |ui| {										
										flow_result(ui, selected_flow);
									});
								
								;
                                
								ui.group(|ui| {                                    
									egui::ScrollArea::vertical().show(ui, |ui| {
										ui.set_width(ui.available_width());
										ui.set_height(ui.available_height());
										flow_locals(ui, selected_flow);
									});            
								});
								
							});							                           
						});

					egui::CentralPanel::default().show_inside(ui, |ui| {
						flow_code_block(ui, selected_flow);
					});
				});
			}
            
        } else {
            ui.heading("No flow selected");
        }
    });
}

fn refs_tool(ui: &mut Ui, state: &mut DebuggerState) {
    ui.heading("REFS");
}

fn taps_tool(ui: &mut Ui, state: &mut DebuggerState) {
    ui.heading("TAPS");
}

fn timeline_tool(ui: &mut Ui, state: &mut DebuggerState) {
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

		fonts.family_and_size.insert(
			TextStyle::Body,
			(egui::FontFamily::Proportional, 15.0)
		);

		fonts.family_and_size.insert(
			TextStyle::Button,
			(egui::FontFamily::Proportional, 15.0)
		);

		ctx.set_fonts(fonts);
		// Enable for debugging widgets layout
		// ctx.set_debug_on_hover(true);

		
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            state_arc,
            ctx_chan_sender: _,
            selected_tool,
        } = self;

        // This is not optimal since we are keeping the lock for the entire frame
        let mut state = state_arc.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("tool_selection_panel").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                    ui.separator();

                    if ui
                        .selectable_label(*selected_tool == DebuggerTool::Flows, "Flows")
                        .clicked()
                    {
                        *selected_tool = DebuggerTool::Flows;
                    } else if ui
                        .selectable_label(*selected_tool == DebuggerTool::Refs, "Refs")
                        .clicked()
                    {
                        *selected_tool = DebuggerTool::Refs;
                    } else if ui
                        .selectable_label(*selected_tool == DebuggerTool::Taps, "Taps")
                        .clicked()
                    {
                        *selected_tool = DebuggerTool::Taps;
                    } else if ui
                        .selectable_label(*selected_tool == DebuggerTool::Timeline, "Timeline")
                        .clicked()
                    {
                        *selected_tool = DebuggerTool::Timeline;
                    }
                });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| match *selected_tool {
                DebuggerTool::Flows => flows_tool(ui, ctx, &mut state),
                DebuggerTool::Refs => refs_tool(ui, &mut state),
                DebuggerTool::Taps => taps_tool(ui, &mut state),
                DebuggerTool::Timeline => timeline_tool(ui, &mut state),
            });

            egui::TopBottomPanel::bottom("bottom_panel").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if let Some(cpu_usage) = frame.info().cpu_usage {
                        ui.label(format!(
                            "Frame duration {:.2} ms ~ {:.0} fps",
                            cpu_usage * 1000.0,
                            1.0 / cpu_usage
                        ));
                    }

                    ui.separator();
                });
            });
        });
    }
}
