use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::{helpers::states::State, Route};

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    // let navigator = use_navigator().unwrap();
    let (state, _) = use_store::<State>();

    // let navigate = |route: Route| {
    //     let navigator = navigator.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         e.prevent_default();
    //         navigator.push(&route);
    //     })
    // };

    html!(
        <header class="top-bar">
            <div class="appName">
                <h1>{ "Vid Conf" }</h1>
            </div>
            <div class="navigators">
                <a href="/">{"Home"}</a>
                <a href="/profile">{"Profile"}</a>
                {
                    if let Some(user ) = &state.current_user {

                        html! {
                                <a href={format!("/profile/{}/{}", &user.username, &user.email)}>{&user.username}</a>
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

#[derive(Properties, Clone, PartialEq)]
struct NavButtonProps {
    text: String,
    route: Route,
}

// navigation buttons
#[function_component]
fn NavButton(props: &NavButtonProps) -> Html {
    let NavButtonProps { text, route } = props;
    let navigator = use_navigator().unwrap();
    let navigate = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&route);
        })
    };

    html! {
        <a onclick={navigate(route.clone())}>{text}</a>
    }
}
