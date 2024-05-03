use std::{cell::RefCell, rc::Rc};

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    constants::{
        cookies::get_cookie,
        user::{get_current_user, set_current_user},
        User, GET_CURRENT_USER_URL,
    },
    Route,
};

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    let navigator = use_navigator().unwrap();
    let current_user = get_current_user();
    use_effect_with(navigator.clone(), |_| {
        if let Some(token) = get_cookie("token") {
            let request = Request::get(&GET_CURRENT_USER_URL)
                .header("Authorization", format!("Bearer {}", token).as_str())
            ;
            let future = async move {
                let response = request.send().await.unwrap();
                let user: User = response.json().await.unwrap();
                set_current_user(user);
            };
            spawn_local(future);
        }
    });

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
                {
                    if let Some(user ) = &current_user {
                        html! {
                            <div>
                                <span>{&user.username}</span>
                                <a href="/logout" onclick={navigate(Route::Profile)}>{"Logout"}</a>
                            </div>
                        }
                    } else {
                        html! {
                            <a href="/login">{"Login"}</a>
                        }
                    }
                }
            </div>
        </header>
    )
}
// {
//     if let Some(user) = &current_user {
//         html!("{}", user.username) // Display username if user is not null
//     } else {
//         html!("Login") // Display "Login" if user is null
//     }
// }
// </a>
