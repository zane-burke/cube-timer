//! Main interface.
//! Consists of both the timer and a shuffling screen.

use crate::history;
use crate::shuffle::{Shuffle, ShuffleDisplay};
use crate::utils;
use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::wasm_bindgen::JsCast;
use yew::{classes, html, Component, Context, Html, KeyboardEvent, Properties};

pub struct Timer {
    pub _time_handler: Interval,           // interval for updating time
    pub start_time: Option<u64>,         // stores the start time of the solve
    pub current_time: String,            // used to display the current time
    pub end_time: Option<u64>,           // stores the time at which the user ends the solve
    pub stage: Stage,                    // the current stage of the solve
    pub toggle_label: String,            // the label used on the toggle button
    pub listener: Option<EventListener>, // event listener for `space` and `r`
    pub shuffle: Shuffle,                // info about the shuffle
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct TimerProps {
    pub dark: bool,
}

pub enum Message {
    ToggleTimer,
    GenerateShuffle,
    Discard,
    UpdateTime,
    KeyPress { event: KeyboardEvent },
}

pub enum Stage {
    Shuffle,
    Inspection,
    Solve,
    Finished,
}

impl Component for Timer {
    type Message = Message;
    type Properties = TimerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let handle = {
            let link = ctx.link().clone();
            Interval::new(10, move || link.send_message(Message::UpdateTime))
        };

        Self {
            _time_handler: handle,
            start_time: None,
            current_time: String::from("00:15.00"),
            end_time: None,
            stage: Stage::Shuffle,
            toggle_label: String::from("Inspect (Space)"),
            listener: None,
            shuffle: Shuffle::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleTimer => {
                self.handle_toggle();

                true
            }
            Message::GenerateShuffle => {
                self.shuffle.generate_shuffle();
                true
            }
            Message::Discard => {
                self.reset();
                true
            }
            Message::UpdateTime => {
                match self.stage {
                    Stage::Inspection => match self.start_time {
                        Some(_) => {
                            self.current_time = {
                                let res = utils::dec_time(self.start_time);

                                if res == 0 {
                                    self.stage = Stage::Solve;
                                    self.start_time = None;
                                    self.toggle_label = String::from("Begin (Space)")
                                }

                                utils::time_string(res)
                            }
                        }
                        None => self.current_time = String::from("00:15.00"),
                    },
                    Stage::Solve => match self.start_time {
                        Some(_) => {
                            self.current_time = utils::time_string(utils::inc_time(self.start_time))
                        }
                        None => self.current_time = String::from("00:00.00"),
                    },
                    // Catches Shuffle and Finished and does nothing with them
                    _ => {}
                };

                true
            }
            Message::KeyPress { event } => {
                match event.key().as_str() {
                    " " => self.handle_toggle(),
                    "r" | "R" => match self.stage {
                        Stage::Shuffle => self.shuffle.generate_shuffle(),
                        _ => self.reset(),
                    },
                    _ => {}
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.stage {
            Stage::Shuffle => self.view_shuffle(ctx),
            _ => self.view_timer(ctx),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let ct = ctx.link().to_owned();
        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw().to_owned();
            ct.send_message(Message::KeyPress { event });
        });

        self.listener.replace(listener);
    }
}

impl Timer {
    fn reset(&mut self) {
        self.stage = Stage::Shuffle;
        self.start_time = None;
        self.end_time = None;
        self.current_time = String::from("00:15.00");
        self.toggle_label = String::from("Inspect (Space)");
    }

    fn handle_toggle(&mut self) {
        match self.stage {
            Stage::Shuffle => {
                self.stage = Stage::Inspection;
            }
            Stage::Inspection => match self.start_time {
                Some(_) => {}
                None => {
                    self.toggle_label = String::from("Inspecting!");
                    self.start_time = Some(utils::get_current_time());
                }
            },
            Stage::Solve => match self.start_time {
                Some(_) => {
                    self.end_time = Some(utils::get_current_time());
                    self.stage = Stage::Finished;
                    self.toggle_label = String::from("Log Solve (Space)");
                }
                None => {
                    self.toggle_label = String::from("Stop (Space)");
                    self.start_time = Some(utils::get_current_time());
                }
            },
            Stage::Finished => {
                let solvetime = utils::saturating_unwrap_sub(self.end_time, self.start_time);
                history::save_solve(history::Solve::new(self.end_time.unwrap(), solvetime, self.shuffle.sequence.join(", ")));
                self.reset();
            }
        }
    }
}

// html rendering
impl Timer {
    fn view_timer(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = ctx.props().dark.then_some("dark");

        let disable_toggle = (&self.toggle_label == "Inspecting!").then_some("disabled");

        html! {
            <>
                <div class={classes!("flex-col-container", dark_mode)}>
                    <span class="span-label">{ &self.stage.as_str() }</span>
                    <span class={classes!("timer", dark_mode)}>{ &self.current_time }</span>
                </div>
                <div class={"flex-row-container"}>
                    <button class={classes!("common-button", "toggle", disable_toggle, dark_mode)} onclick={ctx.link().callback(|_| Message::ToggleTimer)}>{ &self.toggle_label } </button>
                    <button class={classes!("common-button", "danger", dark_mode)} onclick={ctx.link().callback(|_| Message::Discard)}>{ "Reset (r)" } </button>
                </div>
            </>
        }
    }

    fn view_shuffle(&self, ctx: &Context<Self>) -> Html {
        let dm = ctx.props().dark;
        let dark_mode = dm.then_some("dark");

        html! {
            <div class={classes!("flex-col-container", dark_mode)}>
                <div class={classes!("flex-col-container", dark_mode)}>
                    <span class="span-label">{ &self.stage.as_str() }</span>
                    <input
                        type="text"
                        ref={&self.shuffle.node_ref}
                        class={classes!("text-box", dark_mode)}
                        placeholder={self.shuffle.length.to_string()}
                        onchange={ctx.link().callback(|_| Message::GenerateShuffle)}
                    />
                    if !self.shuffle.error.is_empty() {<span class={classes!("err-msg", dark_mode)}>{ &self.shuffle.error }</span>}

                    if !self.shuffle.sequence.is_empty() { <ShuffleDisplay dark={dm} shuffle={utils::chunk_vec(&self.shuffle.sequence)}/> }
                </div>
                <div class={"flex-row-container"}>
                    <button class={classes!("common-button", "toggle", dark_mode)} onclick={ctx.link().callback(|_| Message::ToggleTimer)}>{ "Use and Continue (Space)" } </button>
                    <button class={classes!("common-button", "reset", dark_mode)} onclick={ctx.link().callback(|_| Message::GenerateShuffle)}>{ "Shuffle (r)" } </button>
                </div>
            </div>
        }
    }
}

impl Stage {
    // represent the stage as a `&str`
    pub fn as_str(&self) -> &str {
        match self {
            Stage::Solve => "Solve",
            Stage::Inspection => "Inspection",
            Stage::Finished => "Finished",
            Stage::Shuffle => "Shuffle",
        }
    }
}