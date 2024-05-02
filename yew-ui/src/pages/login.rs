use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{window, HtmlInputElement};

use crate::{constants::{cookies::set_cookie, LOGIN_URL}, Route};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();

    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    
    let onsubmit = {
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = navigator.clone();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();

            // Handle signup submission
            log::info!("login submitted: {}, {}", email, password);

            let sign_data = serde_json::json!({
                "email": email,
                "password": password,
            });

            spawn_local(async move {
                let window = window().expect("no global `window` exists");

                let request = Request::post(&LOGIN_URL)
                   .json(&sign_data)
                   .unwrap();

                let response = request.send().await.unwrap();

                if response.ok() {

                    let confirm = window.confirm_with_message("Signup successful. Proceed to login?");
                    if confirm.unwrap() {
                        navigator.push(&Route::Login);
                    }
                    set_cookie(&window, "token", response.text().await.unwrap().as_str());
                    navigator.push(&Route::Home);
                } else {
                    log::error!("Signup error: {}", response.text().await.unwrap());
                    // display error message
                    let error = window.alert_with_message("Signup failed. Please try again.");
                    if error.is_err() {
                        log::error!("Failed to display error message");
                    }
                }
            });
        })
    };

    html! {
        <div class="flex justify-center items-center content-center flex-col m-auto">
            <div class="flex items-center flex-col">
                <h1>{ "Login" }</h1>
            </div> 
            <form onsubmit={onsubmit}>
                    <div class="form-div">
                        <input 
                        class="text-input"
                        ref={email_ref} 
                        type="text"
                        required={true}
                        placeholder="Username"
                        pattern="^[a-zA-Z0-9]{6,}$"
                         id="username" 
                    />
                    <input 
                        ref={password_ref}
                        class="text-input"
                        label="Password"
                        type="password_ref" 
                        placeholder="Password"
                        required={true}
                        pattern="^[a-zA-Z0-9]{8,}$"
                        id="password"
                    />
                </div>      
                    <input type="submit" value="LOGIN" class="py-2 px-4 pointer bg-yew-blue rounded-md w-full cursor-pointer" 
                    />
                    <p>{"Do not have an account?"}
                    <a href="/signup">{" Sign up"}</a></p>
                </form>
        </div>
    }
}