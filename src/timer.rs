//! Main interface.
//! Consists of both the timer and a shuffling screen.

use crate::shuffle::{Shuffle, ShuffleDisplay};
use crate::utils::{chunk_vec, dec_time, inc_time};
use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::wasm_bindgen::JsCast;
use yew::{classes, html, Component, Context, Html, KeyboardEvent, Properties};

pub struct Timer {
    pub _standalone: Interval,
    pub start_time: Option<u64>,
    pub current_time: String,
    pub stage: Stage,
    pub toggle_label: String,
    pub listener: Option<EventListener>,
    pub shuffle: Shuffle,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct TimerProps {
    pub dark: bool,
}

pub const INSPECTION_TIME: u64 = 15_000;

const SECOND: u64 = 1_000;
const MINUTE: u64 = 60 * SECOND;
const HOUR: u64 = 60 * MINUTE;

pub enum Message {
    ToggleTimer,
    GenerateShuffle,
    LogSolve,
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
            _standalone: handle,
            start_time: None,
            current_time: String::from("00:15.00"),
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
            Message::LogSolve => true,
            Message::Discard => {
                self.reset();
                true
            }
            Message::UpdateTime => {
                match self.stage {
                    Stage::Inspection => match self.start_time {
                        Some(_) => {
                            self.current_time = {
                                let res = dec_time(self.start_time);

                                if res == 0 {
                                    self.stage = Stage::Solve;
                                    self.start_time = None;
                                    self.toggle_label = String::from("Begin (Space)")
                                }

                                time_string(res)
                            }
                        }
                        None => self.current_time = String::from("00:15.00"),
                    },
                    Stage::Solve => match self.start_time {
                        Some(_) => self.current_time = time_string(inc_time(self.start_time)),
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
                    self.start_time = Some(js_sys::Date::new_0().get_time() as u64);
                }
            },
            Stage::Solve => match self.start_time {
                Some(_) => {
                    self.stage = Stage::Finished;
                    self.toggle_label = String::from("Log Solve (Space)");
                }
                None => {
                    self.toggle_label = String::from("Stop (Space)");
                    self.start_time = Some(js_sys::Date::new_0().get_time() as u64);
                }
            },
            Stage::Finished => {
                self.reset();
            }
        }
    }
}

// formats a `u64` time into a pretty string
pub fn time_string(time: u64) -> String {
    let min = (time / MINUTE) % 60; // convert time to the right unit then mod 60 to prevent overflow
    let s = (time / SECOND) % 60;
    let ms = (time % SECOND) / 10;

    if time >= HOUR {
        let hr = time / HOUR;
        return format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hr, min, s, ms).to_string();
    }

    format!("{:0>2}:{:0>2}.{:0>2}", min, s, ms).to_string()
}

// html rendering
impl Timer {
    fn view_timer(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = ctx.props().dark.then_some("dark");

        let disable_toggle = (&self.toggle_label == "Inspecting!").then_some("disabled");

        html! {
            <>
                <div class={classes!("flex-row-container", dark_mode)}>
                    <span class={classes!("timer", dark_mode)}>{ &self.current_time }</span>
                </div>
                <div class={"flex-row-container"}>
                    <button class={classes!("timer-button", "toggle", disable_toggle, dark_mode)} onclick={ctx.link().callback(|_| Message::ToggleTimer)}>{ &self.toggle_label } </button>
                    <button class={classes!("timer-button", "reset", dark_mode)} onclick={ctx.link().callback(|_| Message::Discard)}>{ "Reset (r)" } </button>
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
                    <span class="span-label">{"Shuffle"}</span>
                    <input
                        type="text"
                        ref={&self.shuffle.node_ref}
                        class={classes!("text-box", dark_mode)}
                        placeholder={self.shuffle.length.to_string()}
                        onchange={ctx.link().callback(|_| Message::GenerateShuffle)}
                    />
                    if !self.shuffle.error.is_empty() {<span class={classes!("err-msg", dark_mode)}>{ &self.shuffle.error }</span>}

                    if !self.shuffle.sequence.is_empty() { <ShuffleDisplay dark={dm} shuffle={chunk_vec(&self.shuffle.sequence)}/> }
                </div>
                <div class={"flex-row-container"}>
                    <button class={classes!("timer-button", "toggle", dark_mode)} onclick={ctx.link().callback(|_| Message::ToggleTimer)}>{ "Use and Continue (Space)" } </button>
                    <button class={classes!("timer-button", "reset", dark_mode)} onclick={ctx.link().callback(|_| Message::GenerateShuffle)}>{ "Shuffle (r)" } </button>
                </div>
            </div>
        }
    }
}
