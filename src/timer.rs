use crate::seq_gen;
use gloo::timers::callback::Interval;
use gloo::{events::EventListener, utils::document};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, Node};
use yew::{
    classes, function_component, html, Component, Context, Html,
    KeyboardEvent, NodeRef, Properties,
};

pub struct Timer {
    pub start_time: Option<u64>,
    pub current_time: String,
    pub _standalone: Interval,
    pub stage: Stage,
    pub toggle_label: String,
    pub listener: Option<EventListener>,
    pub shuffle: Vec<String>,
    pub shuffle_length: u64,
    pub shuffle_error: String,
    pub shuffle_ref: NodeRef,
    // pub solve_time:
    //
    //
    //
    //
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct TimerProps {
    pub dark: bool,
}

const INSPECTION_TIME: u64 = 15_000;

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
            start_time: None,
            current_time: String::from("00:15.00"),
            _standalone: handle,
            stage: Stage::Shuffle,
            toggle_label: String::from("Inspect (Space)"),
            listener: None,
            shuffle: seq_gen::shuffler(25), // this should be changed to get a value from local storage to prevent changing the user's preferences
            shuffle_length: 25,
            shuffle_ref: NodeRef::default(),
            shuffle_error: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleTimer => {
                self.handle_toggle();

                true
            }
            Message::GenerateShuffle => {
                self.generate_shuffle();
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
                                let res = self.dec_time();

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
                        Some(_) => self.current_time = time_string(self.inc_time()),
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
                        Stage::Shuffle => self.generate_shuffle(),
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

    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        date.get_time().to_string()
    }

    // decreases time
    fn dec_time(&self) -> u64 {
        let c = js_sys::Date::new_0().get_time() as u64;
        let s = self.start_time.unwrap_or(0) + INSPECTION_TIME;

        saturating_sub(s, c)
    }

    // increases time
    fn inc_time(&self) -> u64 {
        let c = js_sys::Date::new_0().get_time() as u64;
        let s = self.start_time.unwrap_or(0);

        c - s
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
                    self.toggle_label = String::from("Solving!");
                    self.start_time = Some(js_sys::Date::new_0().get_time() as u64);
                }
            },
            Stage::Finished => {
                self.reset();
            }
        }
    }

    fn generate_shuffle(&mut self) {
        let length_ref = &self.shuffle_ref;
        let length_value = length_ref.cast::<HtmlInputElement>().unwrap().value();

        match length_value.parse::<u64>() {
            Ok(r) => {
                self.shuffle_error.clear();
                self.shuffle_length = r;
            },
            Err(e) => {
                if length_value.is_empty() {
                    self.shuffle_length = 25;
                } else {
                    self.shuffle_error = format!("Invalid input: {}", e);
                }
            }
        }

        self.shuffle = seq_gen::shuffler(self.shuffle_length);
    }
}

pub fn saturating_sub(lhs: u64, rhs: u64) -> u64 {
    if rhs > lhs {
        0
    } else {
        lhs - rhs
    }
}

// formats a `u64` time into a pretty string
pub fn time_string(time: u64) -> String {
    let min = (time / MINUTE) % 60; // convert time to the right unit then mod 60 to prevent overflow
    let s = (time / SECOND) % 60; 
    let ms = (time % SECOND) / 10;

    if time >= HOUR {
        let hr = time / HOUR;
        return format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hr, min, s, ms).to_string()
    }

    format!("{:0>2}:{:0>2}.{:0>2}", min, s, ms).to_string()
}

// html rendering
impl Timer {
    fn view_timer(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = ctx.props().dark.then_some("dark");

        html! {
            <>
                <div class={classes!("flex-row-container", dark_mode)}>
                    <span class={classes!("timer", dark_mode)}>{ &self.current_time }</span>
                </div>
                <div class={"flex-row-container"}>
                    <button class={classes!("timer-button", "toggle", dark_mode)} onclick={ctx.link().callback(|_| Message::ToggleTimer)}>{ &self.toggle_label } </button>
                    <button class={classes!("timer-button", "reset", dark_mode)} onclick={ctx.link().callback(|_| Message::Discard)}>{ "Reset (r)" } </button>
                </div>
            </>
        }
    }

    fn view_shuffle(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = ctx.props().dark.then_some("dark");

        html! {
            <div class={classes!("flex-col-container", dark_mode)}>
                <div class={classes!("flex-col-container", dark_mode)}>
                    <span class="span-label">{"Shuffle"}</span>
                    <input
                        type="text"
                        ref={&self.shuffle_ref}
                        class={classes!("text-box", dark_mode)}
                        placeholder="25"
                        onchange={ctx.link().callback(|_| Message::GenerateShuffle)}
                    />
                    if !self.shuffle_error.is_empty() {<span class={classes!("err-msg", dark_mode)}>{ &self.shuffle_error }</span>}

                    if !self.shuffle.is_empty() {
                        <ShuffleDisplay dark={ctx.props().dark} shuffle={split_shuffle_vec(&self.shuffle)}/>
                    }
                </div>
                <div class={"flex-row-container"}>
                    <button class={classes!("timer-button", "toggle", dark_mode)} onclick={ctx.link().callback(|_| Message::ToggleTimer)}>{ "Use and Continue (Space)" } </button>
                    <button class={classes!("timer-button", "reset", dark_mode)} onclick={ctx.link().callback(|_| Message::GenerateShuffle)}>{ "Shuffle (r)" } </button>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ShuffleDisplayProps {
    shuffle: Vec<Vec<String>>,
    dark: bool,
}

#[function_component]
fn ShuffleDisplay(props: &ShuffleDisplayProps) -> Html {
    let shuffle = &props.shuffle;

    let node = {
        let table = document().create_element("table").unwrap();
        table.class_list().add_1("shuffle-table").unwrap();

        if props.dark {
            table.class_list().add_1("dark").unwrap();
        }

        let table_body = document().create_element("tbody").unwrap();

        shuffle.iter().for_each(|row_data| {
            let row = document().create_element("tr").unwrap();

            row_data.iter().for_each(|cell_data| {
                let cell = document().create_element("td").unwrap();
                
                cell.class_list().add_1("shuffle-cell").unwrap();

                if props.dark {
                    cell.class_list().add_1("dark").unwrap();
                }

                cell.append_child(&document().create_text_node(cell_data))
                    .unwrap();
                row.append_child(&cell).unwrap();
            });

            table_body.append_child(&row).unwrap();
        });

        table.append_child(&table_body).unwrap();

        let node: Node = table.into();
        Html::VRef(node)
    };

    node
}

fn split_shuffle_vec(shuffle: &Vec<String>) -> Vec<Vec<String>> {
    let length = shuffle.len();

    shuffle.chunks(5).map(|s| s.into()).collect()
}
