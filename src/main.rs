mod saving;
mod shuffling;

use iced::window::Position;
use iced::{alignment, executor, keyboard, time, window, Length};
use iced::{
    widget::{button, column, container, row, text, Scrollable, Slider},
    Alignment, Application, Command, Element, Font, Settings, Subscription, Theme,
};
use itertools::Itertools;
use shuffling::shuffler;
use std::time::{Duration, Instant};

use chrono::Local;

use saving::Solve;

const INSPECTION_TIME: Duration = Duration::from_secs(15);
const DEFAULT_SHUFFLE_LENGTH: u64 = 25;
const BIG_FONT_SIZE: u16 = 96;
const HOUR: u64 = 3600;
const MINUTE: u64 = 60;

static ICO_DATA: &[u8] = include_bytes!("../images/cube.png");

fn main() -> iced::Result {
    let ico = iced::window::icon::from_file_data(ICO_DATA, None).ok();

    let settings: Settings<()> = Settings {
        window: window::Settings {
            size: iced::Size {
                width: 520.0,
                height: 400.0,
            },
            min_size: Some(iced::Size {
                width: 410.0,
                height: 370.0,
            }),
            resizable: true,
            decorations: true,
            position: Position::default(),
            visible: true,
            icon: ico,
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
    cache: Vec<Solve>,    // stores the shuffle and time of a given solve
    recent: String,       // The formatted string of the 5 most recent solve times
    avg: Duration, // The average of the 5 most recent runs, but excludes the best and worst runs
    atv: Duration, // The all-time average of the save file
    pb: Duration,  // The all-time personal best in the save file
}

#[derive(Default)]
enum State {
    #[default]
    Idle,
    Finished,
    Inspecting(Instant),
    Ticking(Instant),
}

impl Application for CubeTimer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let cache = saving::read_save();
        (
            Self {
                inspection: INSPECTION_TIME,
                finished: false,
                shuffle_length: DEFAULT_SHUFFLE_LENGTH as f64,
                shuffle: shuffler(DEFAULT_SHUFFLE_LENGTH),
                atv: saving::all_time_average(&cache),
                pb: saving::personal_best(&cache),
                cache,
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
                            if !self.finished && self.solve_time > Duration::ZERO {
                                self.finished = true;
                                self.state = State::Finished;
                            } else {
                                self.state = State::Ticking(Instant::now())
                            }
                        }
                        State::Finished => {
                            let new_solve =
                                Solve::new(Local::now(), self.solve_time, self.shuffle.clone());
                            self.cache.push(new_solve);

                            saving::save_solves(&self.cache);

                            self.reset();

                            let last_five: Vec<&Duration> = self
                                .cache
                                .iter()
                                .rev()
                                .take(5)
                                .map(|sv| &sv.solve_time)
                                .collect();

                            let (min, max) =
                                last_five.iter().copied().minmax().into_option().unwrap(); // better than separate min max calls

                            let sum: Duration = last_five.iter().copied().sum();

                            let mut str_accumulator = String::new();
                            let length = last_five.len(); // necessary because take takes UP TO 5 elements, not necessarily always 5

                            if length == 1 {
                                let secs = last_five[0].as_secs();
                                let string = format!(
                                    "{:0>2}:{:0>2}.{:0>2}",
                                    (secs % HOUR) / MINUTE,
                                    secs % MINUTE,
                                    last_five[0].subsec_millis() / 10,
                                );

                                str_accumulator += &string;
                            } else {
                                for t in last_five {
                                    let tsecs = t.as_secs();
                                    str_accumulator += &format!(
                                        "{:0>2}:{:0>2}.{:0>2}",
                                        (tsecs % HOUR) / MINUTE,
                                        tsecs % MINUTE,
                                        t.subsec_millis() / 10,
                                    );

                                    if t == min {
                                        str_accumulator += " (Best)\n";
                                    } else if t == max {
                                        str_accumulator += " (Worst)\n";
                                    } else {
                                        str_accumulator += "\n";
                                    }
                                }
                            }

                            self.recent = str_accumulator;
                            self.avg = match length {
                                1 => Duration::from_secs(0),
                                2 => Duration::from_secs(0),
                                3 => sum - *max - *min, // removes best and worst runs, leaving 1 solve left
                                4 => (sum - *max - *min) / 2,
                                5 => (sum - *max - *min) / 3,
                                _ => unreachable!("The length can't exceed 5"),
                            };
                            self.atv = saving::all_time_average(&self.cache);
                            self.pb = saving::personal_best(&self.cache);
                        }
                        State::Inspecting(..) => {} // empty to prevent user actions other than resetting
                        State::Ticking(..) => {
                            self.state = State::Finished;
                            self.finished = true;
                        }
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
                    self.cache.push(Solve::new(
                        Local::now(),
                        self.solve_time,
                        self.shuffle.clone(),
                    ));
                }

                self.reset()
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = match self.state {
            State::Idle => Subscription::none(),
            State::Finished => Subscription::none(),
            State::Inspecting(..) => {
                time::every(Duration::from_millis(1)).map(Message::InspectionTick)
            }
            State::Ticking(..) => time::every(Duration::from_millis(1)).map(Message::Tick),
        };

        fn handle_hotkey(key: keyboard::Key, _mods: keyboard::Modifiers) -> Option<Message> {
            use keyboard::key;
            match key.as_ref() {
                keyboard::Key::Named(key::Named::Space) => Some(Message::Toggle), // consider adding an if-else block that triggers LogSolve instead of Toggle
                keyboard::Key::Named(key::Named::Escape) => Some(Message::Reset),
                keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::IncrementSlider),
                keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::DecrementSlider),
                _ => None,
            }
        }

        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }

    fn view(&self) -> Element<Message> {
        let font_bf = Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Bold,
            ..Font::default()
        };

        let timer_label = if self.inspection > Duration::ZERO {
            text("Inspection Timer")
                .font(font_bf)
                .horizontal_alignment(alignment::Horizontal::Center)
        } else {
            text("Solve Timer")
                .font(font_bf)
                .horizontal_alignment(alignment::Horizontal::Center)
        };

        let timer = if self.inspection > Duration::ZERO {
            let inspection_time = self.inspection.as_secs();

            text(format!(
                "{:0>2}.{:0>2}",
                inspection_time % MINUTE,
                self.inspection.subsec_millis() / 10,
            ))
            .size(BIG_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Center)
        } else {
            let solve_time = self.solve_time.as_secs();

            text(format!(
                "{:0>2}:{:0>2}.{:0>2}",
                (solve_time % HOUR) / MINUTE,
                solve_time % MINUTE,
                self.solve_time.subsec_millis() / 10,
            ))
            .size(BIG_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Center)
        };

        let toggle_button = {
            let label = if self.inspection == INSPECTION_TIME {
                "Inspection (Space)"
            } else {
                match self.state {
                    State::Idle => "Start Solve (Space)",
                    State::Finished => "Log Solve (Space",
                    State::Inspecting(..) => "Inspecting!",
                    State::Ticking(..) => "Stop (Space)",
                }
            };

            button(label).on_press(Message::Toggle).width(Length::Fill)
        };

        let discard_button = button("Discard and Reset (Esc)")
            .on_press(Message::Reset)
            .width(Length::Fill);

        let controls = column![toggle_button, discard_button].spacing(10);

        let avg_label = text("Last Five Avg: ").font(font_bf);
        let avg_secs = self.avg.as_secs();
        let avg_display = text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (avg_secs % HOUR) / MINUTE,
            avg_secs % MINUTE,
            self.avg.subsec_millis() / 10,
        ));
        
        let atv_label = text("All Time Avg: ").font(font_bf);
        let atv_secs = self.atv.as_secs();
        let atv_display = text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (atv_secs % HOUR) / MINUTE,
            atv_secs % MINUTE,
            self.atv.subsec_millis() / 10,
        ));
        
        let pb_label = text("All-Time PB: ").font(font_bf);
        let pb_secs = self.pb.as_secs();
        let pb_display = text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (pb_secs % HOUR) / MINUTE,
            pb_secs % MINUTE,
            self.atv.subsec_millis() / 10,
        ));

        let time_labels = container(column![pb_label, atv_label, avg_label].spacing(10));
        let time_values = container(column![pb_display, atv_display, avg_display].spacing(10));
        let times_container = container(row![time_labels, time_values].spacing(10)).width(Length::Fill);

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

        column![
            timer_label,
            timer,
            row![
                column![controls, shuffle].spacing(10),
                column![times_container, five_container].spacing(10)
            ]
            .spacing(10)
        ]
        .padding(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }
}

impl CubeTimer {
    fn reset(&mut self) {
        self.inspection = INSPECTION_TIME;
        self.solve_time = Duration::default();
        self.finished = false;
        self.shuffle = shuffler(self.shuffle_length as u64);
    }
}
