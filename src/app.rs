use yew::{classes, html::Scope, Component, Context, Html};
use yew::html;
use yew_router::{prelude::Link, BrowserRouter, Routable, Switch};

use crate::preferences;
use crate::stats::Stats;
use crate::timer::Timer;
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

pub struct App {
    pub dark: bool,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark: preferences::get_theme(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleDarkMode => {
                self.dark = !self.dark;
                preferences::set_theme(self.dark);
                true
            }
        }
    }

    // This should probably be changed to use ContextProvider, but I don't want to deal with changing things to function components
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="root">
                    <BrowserRouter>
                        <main class={classes!("root", self.dark.then_some("dark"))}>
                            { self.render_navbar(ctx.link()) }
                            if self.dark {<Switch<Route> render={dark_switch} />}
                            else {<Switch<Route> render={switch} />}
                        </main>
                    </BrowserRouter>
            </div>
        }
    }
}

impl App {
    fn render_navbar(&self, link: &Scope<Self>) -> Html {
        let dm = preferences::get_theme();
        let dark_mode = dm.then_some("dark");

        let btn_ico = if !dm { "fa-sun-o" } else { "fa-moon-o" };

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

// switch routes using light mode
fn switch(routes: Route) -> Html {
    match routes {
        Route::Timer => html! { <Timer dark={false}/>},
        Route::Stats => html! { <Stats dark={false}/>},
        Route::NotFound { .. } => html! { <h1>{ "404" }</h1> },
    }
}

// switch routes using light mode
fn dark_switch(routes: Route) -> Html {
    match routes {
        Route::Timer => html! { <Timer dark={true}/>},
        Route::Stats => html! { <Stats dark={true}/>},
        Route::NotFound { .. } => html! { <h1>{ "404" }</h1> },
    }
}

