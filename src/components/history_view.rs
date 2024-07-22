use yew::{classes, function_component, html, Html, Properties};

use crate::history::{self, Solve};

use super::solve_display::SolveDisplay;

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