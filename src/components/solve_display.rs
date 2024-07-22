use yew::{function_component, html, Html, classes, Properties};

use crate::{history::Solve, utils};

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