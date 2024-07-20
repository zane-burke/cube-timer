use yew::{classes,  html, Component, Context, Html, Properties};

pub struct Stats;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct StatProps {
    pub dark: bool,
}

pub enum Message {
    GenerateShuffle,
}

impl Component for Stats {
    type Message = Message;
    type Properties = StatProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::GenerateShuffle => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = ctx.props().dark.then_some("dark");

        html! {
            <div class="flex-col-container">
                <span>{ "All-Time PB:" }</span>
                <span>{ "All-Time Average:" }</span>
                <span>{ "Last Five Average:" }</span>
            </div>
        }
    }
}



