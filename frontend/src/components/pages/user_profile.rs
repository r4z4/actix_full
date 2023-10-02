use std::ops::Deref;

use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::prelude::use_store;
use reqwasm::http::Request;

use crate::store::AuthStore;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserProfileData {
    user_id: i32,
    account_id: i32,
    email: String,
    username: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseUserProfile {
    user: UserProfileData,
}

#[function_component(UserProfile)]
pub fn user_profile() -> Html {
    let (store, dispatch) = use_store::<AuthStore>();
    let data: UseStateHandle<Option<UserProfileData>> = use_state(|| None);
    let user_id = store.user_id.unwrap_or(1);

    let c_data = data.clone();
    let view_data = data.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&format!("http://localhost:8000/user/{}", user_id))
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseUserProfile>()
                    .await
                    .unwrap();
        
                c_data.set(Some(response.user));
            });
        },
        data,
    );

    
    html! {
        <div class={"entity-page"}>
            <h1>{format!("User Profile for {}", store.username.clone().unwrap())}</h1>
            if view_data.is_some() {
                <div class={"container"}>
                    <p>{"Username"}</p>
                    <p>{view_data.as_ref().unwrap().email.clone()}</p>
                    // <UserProfileTable />
                </div>
            }
        </div>
    }
}
