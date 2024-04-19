use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::route::Route;
const TEXT_INPUT_CLASSES: &str = "rounded-md mx-2 p-2 text-black required:ring-2 required:ring-red-500 required:valid:ring-2 required:valid:ring-green-500";

#[function_component(Login)]
pub fn login() -> Html {
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
        <div class="flex justify-center items-center content-center flex-col m-auto">
            <div class="flex items-center flex-col">
                <h1>{ "Login" }</h1>
            </div> 
            <form onsubmit={onsubmit}>
                    <div class="py-4">
                        <input 
                        class={TEXT_INPUT_CLASSES}
                        ref={username_ref} 
                        type="text"
                        required={true}
                        placeholder="Username"
                        pattern="^[a-zA-Z0-9]{6,}$"
                         id="username" 
                    />
                    <input 
                        ref={password_ref}
                        class={TEXT_INPUT_CLASSES}
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
                </form>
        </div>
    }
}