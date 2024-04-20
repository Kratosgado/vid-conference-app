use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    let navigator = use_navigator().unwrap();

    let navigate = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&route);
        })
    };
    
    html!(
        <header class="top-bar">
            <div class="appName">
                <h1>{ "Vid Conf" }</h1>
            </div>
            <div class="navigators">
                <a href="/">{"Home"}</a>
                <a href="/profile">{"Profile"}</a>
                <a href="/login">{"Login"}</a>
              
            </div>
        </header>
    )
}