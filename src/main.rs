#![recursion_limit = "1024"]

mod saving;
mod seq_gen;
mod stats;
mod timer;
mod utils;
mod shuffle;

use yew::html;
use yew::{classes, html::Scope, Component, Context, Html};
use yew_router::{prelude::Link, BrowserRouter, Routable, Switch};

use crate::stats::Stats;
use crate::timer::Timer;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Timer,
    #[at("/stats")]
    Stats,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Message {
    ToggleDarkMode,
}

#[derive(Default)]
pub struct App {
    pub dark_mode: bool,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleDarkMode => {
                self.dark_mode = !self.dark_mode;
                true
            }
        }
    }

    // This should probably be changed to use ContextProvider, but I don't want to deal with changing things to function components
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="root">
                <BrowserRouter>
                    <main class={classes!("root", self.dark_mode.then_some("dark"))}>
                        { self.render_navbar(ctx.link()) }
                        if self.dark_mode { 
                            <Switch<Route> render={dark_switch} />
                        } else { 
                            <Switch<Route> render={switch} /> 
                        }
                    </main>
                </BrowserRouter>
            </div>
        }
    }
}

impl App {
    fn render_navbar(&self, link: &Scope<Self>) -> Html {
        let dark_mode = self.dark_mode.then_some("dark");

        let btn_ico = if self.dark_mode {
            "fa-sun-o"
        } else {
            "fa-moon-o"
        };

        let onclick = link.callback(|_| Message::ToggleDarkMode);

        html! {
            <nav class={classes!("navbar", dark_mode)}>
                <Link<Route> classes={classes!("navbar-tab", dark_mode)} to={Route::Timer}>
                    { "Home" }
                </Link<Route>>
                <Link<Route> classes={classes!("navbar-tab", dark_mode)} to={Route::Stats}>
                    { "Stats" }
                </Link<Route>>
                <button {onclick} class={classes!("theme-button", "fa", btn_ico, dark_mode)}>{ "" }</button>
            </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Timer => html! { <Timer dark={false}/>},
        Route::Stats => html! { <Stats dark={false}/>},
        Route::NotFound { .. } => html! { <h1>{ "404" }</h1> },
    }
}

fn dark_switch(routes: Route) -> Html {
    match routes {
        Route::Timer => html! { <Timer dark={true}/>},
        Route::Stats => html! { <Stats dark={true}/>},
        Route::NotFound { .. } => html! { <h1>{ "404" }</h1> },
    }
}
