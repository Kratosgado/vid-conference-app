use crate::{constants::GET_CURRENT_USER_URL, Html};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html};
use yewdux::{init_listener, use_store, Dispatch, Listener, Store};

#[derive(Default, Clone, PartialEq, Store)]
#[store(storage = "local")]
pub struct State {
    pub current_user: Option<User>,
}

#[derive(Default, Clone, PartialEq)]
pub struct CookiesState {
    pub token: String,
}

impl Store for CookiesState {
    fn new(cx: &yewdux::Context) -> Self {
        init_listener(CookiesListener, cx);
        Default::default()
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

pub struct CookiesListener;
impl Listener for CookiesListener {
    type Store = CookiesState;

    fn on_change(&mut self, ctx: &yewdux::Context, state: std::rc::Rc<Self::Store>) {
        let token = state.token.clone();
        log::info!("CookiesState changed: {:?}", state.clone().token);
        if !token.is_empty() {
            let request = Request::get(&GET_CURRENT_USER_URL)
                .header("Authorization", format!("Bearer {}", token).as_str());
            let future = async move {
                let response = request.send().await.unwrap();
                let user: User = response.json().await.unwrap();
                let dispatch = Dispatch::<State>::global();
                dispatch.set(State {
                    current_user: Some(user),
                });
            };
            spawn_local(future);
        }
    }
}

#[function_component]
pub fn GetUsername() -> Html {
    let (state, _) = use_store::<State>();
    let current_user = state.current_user.clone().unwrap();
    html!(current_user.username)
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}
