//! Stat-tracking interface

use crate::history;
use crate::history::Solve;
use crate::utils;
use yew::{classes, function_component};
use yew::{html, Component, Context, Html, Properties};

pub struct Stats {
    pub show_confirmation: bool,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct StatProps {
    pub dark: bool,
}
pub enum Message {
    DeleteHistory,
    ToggleConfirmation,
}

impl Component for Stats {
    type Message = Message;
    type Properties = StatProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_confirmation: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::DeleteHistory => {
                history::set_history(history::History::new());
                self.show_confirmation = !self.show_confirmation;
                true
            }
            Message::ToggleConfirmation => {
                self.show_confirmation = !self.show_confirmation;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark = ctx.props().dark;
        let dark_mode = dark.then_some("dark");

        let toggle = ctx.link().callback(|_| Message::ToggleConfirmation);
        let delete = ctx.link().callback(|_| Message::DeleteHistory);

        html! {
            <div class="flex-col-container">
                <span class="span-label">{ "Stats" }</span>
                <table class="stats-table">
                    <tbody>
                        <tr>
                            <td>{"PB:"}</td>
                            <td class="time-display">{ utils::time_string(history::get_pb()) }</td>
                        </tr>
                        <tr>
                            <td>{ "Average: " }</td>
                            <td class="time-display">{ utils::time_string(history::get_avg()) }</td>
                        </tr>
                        <tr>
                            <td>{ "AO5: " }</td>
                            <td class="time-display">{ utils::time_string(history::get_ao5()) }</td>
                        </tr>
                        <tr>
                            <td>{ "AO50: " }</td>
                            <td class="time-display">{ utils::time_string(history::get_ao(50)) }</td>
                        </tr>
                        <tr>
                            <td>{ "AO100: " }</td>
                            <td class="time-display">{ utils::time_string(history::get_ao(100)) }</td>
                        </tr>
                    </tbody>
                </table>
                if !self.show_confirmation {
                    <button class={classes!("common-button", "danger", dark_mode)} onclick={toggle}>{ "Clear History" }</button>
                } else {
                    <span>{ "This is a destructive operation. Are you sure?"}</span>
                    <div class="flex-row-container">
                        <button class={classes!("common-button", "danger", dark_mode)} onclick={delete}>{ "Clear History" }</button>
                        <button class={classes!("common-button", dark_mode)} onclick={toggle}>{ "Cancel" }</button>
                    </div>
                }

                <span class="span-label">{ "Solve History" }</span>
                <HistoryView {dark} />
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct HistoryViewProps {
    pub dark: bool,
}

#[function_component]
pub fn HistoryView(props: &HistoryViewProps) -> Html {
    let dark = props.dark;
    let dark_mode = dark.then_some("dark");
    let history = history::retrieve_history()
        .history
        .into_iter()
        .rev()
        .collect::<Vec<Solve>>();

    html! {
        <table class="solve-table">
            <tr class="solve-row">
                <td class={classes!("solve-cell", dark_mode)}>
                    { "DD/MM/YY" }
                </td>
                <td class={classes!("time-display", "solve-cell", dark_mode)}>
                    { "Time" }
                </td>
            </tr>
            {
                history.into_iter().map(|solve| {
                    html! {
                        <SolveDisplay {solve} {dark}/>
                    }
                }).collect::<Html>()
            }
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct SolveProps {
    pub solve: Solve,
    pub dark: bool,
}

#[function_component]
pub fn SolveDisplay(props: &SolveProps) -> Html {
    let dark_mode = props.dark.then_some("dark");

    html! {
        <tr class="solve-row">
            <td class={classes!("solve-cell", dark_mode)}>
                { utils::date_string(props.solve.timestamp) }
            </td>
            <td class={classes!("time-display", "solve-cell", dark_mode)}>
                { utils::time_string(props.solve.solvetime) }
            </td>
        </tr>
    }
}
