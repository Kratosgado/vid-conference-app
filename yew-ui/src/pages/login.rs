use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::router::Route;

#[function_component(Login)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let onsubmit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        Callback::from(move |_| {
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
    
            if username == "admin" && password == "admin" {
                navigator.push(&Route::Home);
            } else {
                log::info!("Invalid username or password_ref");
            }
        })
    };

    html! {
        <div>
            <h1>{ "Login" }</h1>
            <form onsubmit={onsubmit}>
                <label>{ "Username" }</label>
                <input ref={username_ref} type="text" id="username" />
                <input 
                ref={password_ref}
                 label="Password"
                  type="password_ref" 
                  placeholder="Password"
                  required={true}
                  pattern="^[a-zA-Z0-9]{8,}$"
                  id="password_ref" />
                <input type="submit" value="LOGIN"/>
            </form>
        </div>
    }
}