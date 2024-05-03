use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::constants::GET_USERS_URL;

use super::states::User;



// // fetch user from backend
// pub fn fetch_user(id: &String) -> Option<User> {
//     let user: Option<User> = None;
//     let future = async move {
//         let request = Request::get(format!("{}/{}", *GET_USERS_URL, id).as_str());
//         // .header("Authorization", format!("Bearer {}", token).as_str());
//     let response = request.send().await;
//       match response {
//           Ok(response) => {
//                user: User = response.json().await.unwrap();
//               Some(user)
//           },
//           Err(_) => None
//       }
//     };
//     spawn_local(future)
//     user
// }