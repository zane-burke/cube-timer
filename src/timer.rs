use crate::utils::{dec_time, inc_time};
use crate::shuffle::Shuffle;
use gloo::timers::callback::Interval;
use gloo::{events::EventListener, utils::document};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{wasm_bindgen::JsCast, Node};
use yew::{
    classes, function_component, html, Component, Context, Html,
    KeyboardEvent, Properties,
};

pub struct Timer {
    pub start_time: Option<u64>,
    pub current_time: String,
    pub _standalone: Interval,
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
            start_time: None,
            current_time: String::from("00:15.00"),
            _standalone: handle,
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
        return format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hr, min, s, ms).to_string()
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
        let dark_mode = ctx.props().dark.then_some("dark");

        html! {
            <div class={classes!("flex-col-container", dark_mode)}>
                <div class={classes!("flex-col-container", dark_mode)}>
                    <span class="span-label">{"Shuffle"}</span>
                    <input
                        type="text"
                        ref={&self.shuffle.node_ref}
                        class={classes!("text-box", dark_mode)}
                        placeholder="25"
                        onchange={ctx.link().callback(|_| Message::GenerateShuffle)}
                    />
                    if !self.shuffle.error.is_empty() {<span class={classes!("err-msg", dark_mode)}>{ &self.shuffle.error }</span>}

                    if !self.shuffle.sequence.is_empty() {
                        <ShuffleDisplay dark={ctx.props().dark} shuffle={split_shuffle_vec(&self.shuffle.sequence)}/>
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
    shuffle.chunks(5).map(|s| s.into()).collect()
}
