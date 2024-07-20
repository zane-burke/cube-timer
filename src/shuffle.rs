use yew::{classes, html, AttrValue, Callback, Component, Context, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ShuffleProps {
    pub onchange: Callback<()>,
    pub placeholder: AttrValue,
    pub input_ref: NodeRef,
    pub dark: bool,
}

pub struct Shuffle;

enum Message {
    Changed,
}

impl Component for Shuffle {
    type Message = Message;
    type Properties = ShuffleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Changed => {
                ctx.props().onchange.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = ctx.props().dark.then_some("dark");
        let placeholder = ctx.props().placeholder.clone();

        html! {
            <input
                ref={&ctx.props().input_ref}
                type="text"
                class={classes!("text-box", dark_mode)}
                placeholder={placeholder}
                onchange={ctx.link().callback(|_| Message::Changed)}
                />
        }
    }
}