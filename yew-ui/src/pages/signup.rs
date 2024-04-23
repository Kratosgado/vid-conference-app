use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(Signup)]
pub fn signup() -> Html {
    let navigator = use_navigator().unwrap();

    let username_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let onsubmit = {
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        Callback::from(move |_| {
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();

            // Handle signup submission
            log::info!("Signup submitted: {}, {}, {}", username, email, password);
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div class="flex justify-center items-center content-center flex-col m-auto">
            <div class="flex items-center flex-col">
                <h1>{ "Signup" }</h1>
            </div>
            <form onsubmit={onsubmit}>
                <div>
                    <input
                        class="text-input"
                        ref={username_ref}
                        type="text"
                        required={true}
                        placeholder="Username"
                        pattern="^[a-zA-Z0-9]{6,}$"
                        id="username"
                    />
                    <input
                        ref={email_ref}
                        class="text-input"
                        label="Email"
                        type="email"
                        placeholder="Email"
                        required={true}
                        pattern="^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+[a-zA-Z]{2,}$"
                        id="email"
                    />
                    <input
                        ref={password_ref}
                        class="text-input"
                        label="Password"
                        type="password"
                        placeholder="Password"
                        required={true}
                        pattern="^[a-zA-Z0-9]{8,}$"
                        id="password"
                    />
                </div>
                <input type="submit" value="SIGNUP" class="button"
                />
                <p>{"Already have an account?"}
                    <a href="/signup">{" Log in"}</a></p>
            </form>
        </div>
    }
}
