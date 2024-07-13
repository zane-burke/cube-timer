mod shuffling;

use iced::widget::{container, Scrollable};
use iced::window::Position;
use iced::{alignment, executor, keyboard, time, window, Length};
use iced::{
    widget::{button, column, row, text, Slider},
    Application, Command, Element, Font, Settings, Subscription, Theme,
};
use shuffling::shuffler;
use std::time::{Duration, Instant};

use chrono::{DateTime, Local};

const INSPECTION_TIME: Duration = Duration::from_secs(15);

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: iced::Size {
                width: 500.0,
                height: 400.0,
            },
            min_size: Some(iced::Size {
                width: 440.0,
                height: 360.0,
            }),
            resizable: true,
            decorations: true,
            position: Position::default(),
            visible: true,
            ..Default::default()
        },
        ..Default::default()
    };

    CubeTimer::run(settings)
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    LogSolve,
    Tick(Instant),
    InspectionTick(Instant),
    SliderChanged(f64),
    IncrementSlider,
    DecrementSlider,
}

#[derive(Default)]
struct CubeTimer {
    inspection: Duration, // the amount of time left in inspection
    solve_time: Duration, // the amount of time the solve took
    state: State,         // controls the activity of the timer
    finished: bool,       // whether the solve is done
    shuffle_length: f64,  // the length of the current shuffle
    shuffle: String,      // the current shuffle
    cache: Vec<(DateTime<Local>, Duration, String)>, // stores the shuffle and time of a given solve
    recent: String,
    avg: Duration,
}

#[derive(Default)]
enum State {
    #[default]
    Idle,
    Inspecting(Instant),
    Ticking(Instant),
}

impl Application for CubeTimer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                inspection: INSPECTION_TIME,
                finished: false,
                shuffle_length: 25f64,
                shuffle: shuffler(25),
                recent: String::from("\n\n\n\n\n"),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cube Timer")
    }

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
            Message::Toggle => {
                if self.inspection > Duration::ZERO {
                    self.state = State::Inspecting(Instant::now());
                } else {
                    match self.state {
                        State::Idle => {
                            if self.finished == true {
                            } else {
                                if self.finished == false && self.solve_time > Duration::ZERO {
                                    self.finished = true;
                                } else {
                                    self.state = State::Ticking(Instant::now())
                                }
                            }
                        }
                        State::Inspecting(..) => {}
                        State::Ticking(..) => self.state = State::Idle,
                    }
                }
            }
            Message::Tick(now) => {
                if let State::Ticking(last) = &mut self.state {
                    self.solve_time += now - *last;
                    *last = now;
                }
            }
            Message::InspectionTick(now) => {
                if let State::Inspecting(last) = &mut self.state {
                    self.inspection = self.inspection.saturating_sub(now - *last);
                    *last = now;

                    if self.inspection == Duration::ZERO {
                        self.state = State::Idle;
                    }
                }
            }
            Message::SliderChanged(value) => {
                self.shuffle_length = value;
                self.shuffle = shuffler(self.shuffle_length as u64);
            }
            Message::IncrementSlider => {
                if self.shuffle_length != 100.0 {
                    self.shuffle_length += 1.0;
                    self.shuffle = shuffler(self.shuffle_length as u64);
                }
            }
            Message::DecrementSlider => {
                if self.shuffle_length != 1.0 {
                    self.shuffle_length -= 1.0;
                    self.shuffle = shuffler(self.shuffle_length as u64);
                }
            }
            Message::Reset => {
                self.state = State::Idle;

                if self.solve_time > Duration::ZERO {
                    self.cache
                        .push((Local::now(), self.solve_time, self.shuffle.clone()));
                }

                self.inspection = INSPECTION_TIME;
                self.solve_time = Duration::default();
                self.finished = false;
                self.shuffle = shuffler(self.shuffle_length as u64);
                // Add:
                // - Thing to cache results
                // - Thing to reset the previously given shuffle.
            }
            Message::LogSolve => {
                self.cache
                    .push((Local::now(), self.solve_time, self.shuffle.clone()));
                self.inspection = INSPECTION_TIME;
                self.solve_time = Duration::default();
                self.finished = false;
                self.shuffle = shuffler(self.shuffle_length as u64);

                let mut last_five: Vec<&Duration> = self
                    .cache
                    .iter()
                    .rev()
                    .take(5)
                    .map(|(_t, st, _sh)| st)
                    .collect();

                last_five.sort();

                let length = last_five.len();
                let sum: Duration = last_five.iter().copied().sum();

                let mut str_accumulator = String::new();

                if length == 1 {
                    let secs = last_five[0].as_secs();
                    let string = format!(
                        "{:0>2}:{:0>2}.{:0>2}",
                        (secs % 60) / 60,
                        secs % 60,
                        last_five[0].subsec_millis() / 10,
                    );

                    str_accumulator += &string;
                } else {
                    for (i, t) in last_five.into_iter().enumerate() {
                        let tsecs = t.as_secs();
                        str_accumulator += &format!(
                            "{:0>2}:{:0>2}.{:0>2}",
                            (tsecs % 60) / 60,
                            tsecs % 60,
                            t.subsec_millis() / 10,
                        );

                        if i == 0 {
                            str_accumulator += " (Best)\n";
                        } else if i == length - 1 {
                            str_accumulator += " (Worst)\n";
                        } else {
                            str_accumulator += "\n";
                        }
                    }
                }

                self.recent = str_accumulator;
                self.avg = sum / length as u32;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = match self.state {
            State::Idle => Subscription::none(),
            State::Inspecting(..) => {
                time::every(Duration::from_millis(1)).map(Message::InspectionTick)
            }
            State::Ticking(..) => time::every(Duration::from_millis(1)).map(Message::Tick),
        };

        fn handle_hotkey(key: keyboard::Key, _mods: keyboard::Modifiers) -> Option<Message> {
            use keyboard::key;
            match key.as_ref() {
                keyboard::Key::Named(key::Named::Space) => Some(Message::Toggle),
                keyboard::Key::Named(key::Named::Escape) => Some(Message::Reset),
                keyboard::Key::Character("l") => Some(Message::LogSolve),
                keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::IncrementSlider),
                keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::DecrementSlider),
                _ => None,
            }
        }

        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }

    fn view(&self) -> Element<Message> {
        const MIN: u64 = 60;
        const HR: u64 = 60 * MIN;

        let font_bf = Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Bold,
            ..Font::default()
        };

        let inspection_label = text("Inspection Timer")
            .font(font_bf)
            .horizontal_alignment(alignment::Horizontal::Left);

        let inspection_time = self.inspection.as_secs();

        let inspection_timer = text(format!(
            "{:0>2}.{:0>2}",
            inspection_time % MIN,
            self.inspection.subsec_millis() / 10,
        ))
        .horizontal_alignment(alignment::Horizontal::Left);

        let solve_label = text("Solve Timer")
            .font(font_bf)
            .horizontal_alignment(alignment::Horizontal::Left);

        let solve_time = self.solve_time.as_secs();

        let solve_timer = text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (solve_time % HR) / MIN,
            solve_time % MIN,
            self.solve_time.subsec_millis() / 10,
        ))
        .horizontal_alignment(alignment::Horizontal::Left);

        let button = |label| button(text(label)).padding(10).width(Length::Fill);

        let toggle_button = {
            let label = if self.inspection == INSPECTION_TIME {
                "Inspection (Space)"
            } else {
                match self.state {
                    State::Idle => "Start Solve (Space)",
                    State::Inspecting(..) => "Inspecting!",
                    State::Ticking(..) => "Stop (Space)",
                }
            };

            button(label).on_press(Message::Toggle).width(Length::Fill)
        };

        let discard_button = button("Discard and Reset (Esc)").on_press(Message::Reset);

        let log_button = button("Log and Reset (L)").on_press(Message::LogSolve);

        let controls = column![toggle_button, log_button, discard_button].spacing(10);

        let avg_label = text("Average:").font(font_bf);

        let avg_secs = self.avg.as_secs();

        let avg_display = text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (avg_secs % HR) / MIN,
            avg_secs % MIN,
            self.avg.subsec_millis() / 10,
        ));

        let avg_container = container(column![avg_label, avg_display]).width(Length::Fill);

        let five_label = text("Recent Solves:").font(font_bf);
        let five_content = text(&self.recent);

        let five_container = container(column![five_label, five_content]).width(Length::Fill);

        let shuffle = column![
            Slider::new(5.0..=100.0, self.shuffle_length, Message::SliderChanged),
            text(format!("Shuffle ({}):", self.shuffle_length)).font(font_bf),
            Scrollable::new(
                column![text(&self.shuffle)].padding(iced::Padding::from([0, 15, 0, 0]))
            ),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let keybind_text =
            text("You can use the left and right arrows to change the shuffle length.");

        row![
            column![
                inspection_label,
                inspection_timer,
                solve_label,
                solve_timer,
                controls,
                keybind_text,
            ]
            .spacing(10),
            column![avg_container, five_container, shuffle].spacing(10)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }
}
