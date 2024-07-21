//! Stat-tracking interface

use crate::history;
use crate::utils;
use yew::classes;
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
                            <td>{"All-time PB:"}</td>
                            <td>{ utils::time_string(history::get_pb()) }</td>
                        </tr>
                        <tr>
                            <td>{ "All-Time Average: " }</td>
                            <td>{ utils::time_string(history::get_avg()) }</td>
                        </tr>
                        <tr>
                            <td>{ "Last Five Average: " }</td>
                            <td>{ utils::time_string(history::get_ao5()) }</td>
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
            </div>
        }
    }
}
