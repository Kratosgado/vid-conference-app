use lazy_static;
use serde::{Deserialize, Serialize};

// Define the constants
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}


lazy_static::lazy_static!(
    pub static ref BACKEND_URL: String = std::env::var("BACKEND_URL").unwrap_or("http://localhost:8000".to_string());
    pub static ref LOGIN_URL: String = format!("{}/users/login", *BACKEND_URL);
    pub static ref SIGNUP_URL: String = format!("{}/users/signup", *BACKEND_URL);
    pub static ref LOGOUT_URL: String = format!("{}/users/logout", *BACKEND_URL);
    pub static ref GET_CURRENT_USER_URL: String = format!("{}/users/me", *BACKEND_URL);
    pub static ref GET_USERS_URL: String = format!("{}/users", *BACKEND_URL);
    pub static ref DELETE_USER_URL: String = format!("{}/users", *BACKEND_URL);
    // define a mutable static variable
    pub static ref CURRENT_USER: std::sync::Mutex<Option<User>> = std::sync::Mutex::new(None);
);

pub mod cookies {
    use web_sys::window;

    pub fn set_cookie(window: &web_sys::Window, key: &str, value: &str) {
        let storage = window.local_storage().expect("no local storage").unwrap();
        storage.set_item(key, value).expect("failed to write cookie");
    }

    pub fn get_cookie(key: &str) -> Option<String> {
        let window = window().expect("no global `window` exists");
        let storage = window.local_storage().expect("no local storage").unwrap();
        storage.get_item(key).expect("failed to read cookie")
    }
}

pub mod user {
    use super::User;
    use super::CURRENT_USER;

    pub fn set_current_user(user: User) {
        let mut current_user = CURRENT_USER.lock().unwrap();
        *current_user = Some(user);
    }

    pub fn get_current_user() -> Option<User> {
        let current_user = CURRENT_USER.lock().unwrap();
        current_user.clone()
    }
}