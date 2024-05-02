use lazy_static;

lazy_static::lazy_static!(
    pub static ref BACKEND_URL: String = "http://localhost:8000".to_string();
    pub static ref LOGIN_URL: String = format!("{}/users/login", *BACKEND_URL);
    pub static ref SIGNUP_URL: String = format!("{}/users/signup", *BACKEND_URL);
    pub static ref LOGOUT_URL: String = format!("{}/users/logout", *BACKEND_URL);
    pub static ref GET_USERS_URL: String = format!("{}/users", *BACKEND_URL);
    pub static ref DELETE_USER_URL: String = format!("{}/users", *BACKEND_URL);
);
