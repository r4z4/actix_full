use gloo::console::log;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::functional::use_store;
use yewdux::prelude::*;
use yewdux::store::*;

use crate::components::button::Button;
use crate::router::{switch, Route};
use crate::store::AuthStore;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiLoginResponse {
    pub id: i32,
    pub username: String,
    pub token: String,
}

pub async fn login_user(username: String, password: String) -> ApiLoginResponse {
    let body = json!({
        "username": username,
        "password": password
    });
    let response = Request::post("http://localhost:3000/users/login")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiLoginResponse>()
        .await
        .unwrap();

    response
}

// const real_login_form_submit = Callback::from(|event: SubmitEvent| {
//     event.prevent_default();
//     let username = state.username.clone();
//     let password = state.password.clone();
//     wasm_bindgen_futures::spawn_local(async move {
//         let response = login_user(username, password).await;
//         log!(response.token)
//     })
// });

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let (store, dispatch) = use_store::<AuthStore>();
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    let navigator = use_navigator().unwrap();

    let onchange_username = {
        let cloned_data_state = state.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |event: Event| {
            let username: String = event.target_unchecked_into::<HtmlInputElement>().value();
            let username: Option<String> = if username.is_empty() {
                None
            } else {
                Some(username)
            };
            let cloned_username = username.clone();
            dispatch.reduce_mut(|store| store.username = username);
            let mut data = cloned_data_state.deref().clone();
            data.username = cloned_username.unwrap();
            cloned_data_state.set(data);
        })
    };

    let onchange_password = {
        let cloned_data_state = state.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |event: Event| {
            let password: String = event.target_unchecked_into::<HtmlInputElement>().value();
            let password: Option<String> = if password.is_empty() {
                None
            } else {
                Some(password)
            };
            let cloned_password = password.clone();
            // dispatch.reduce_mut(|store| store.password = password);
            let mut data = cloned_data_state.deref().clone();
            data.password = cloned_password.unwrap();
            cloned_data_state.set(data);
        })
    };

    // let form_onsubmit = real_login_form_submit.clone();
    let cloned_state = state.clone();

    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();
        event.prevent_default();
        let username = state.username.clone();
        let password = state.password.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = login_user(username, password).await;
            // Use this
            // log!(response.token)
            dispatch.reduce_mut(|store| store.token = Some(response.token));
            navigator.push(&Route::Home);
        })
    });

    // let token = if let Some(state) = store.state() {
    //     state.token.clone()
    // } else {
    //     String::new() // Just get new empty string
    // };

    html! {
        <div>
            <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                <input type="text" placeholder="Username" onchange={onchange_username} />
                <input type="text" placeholder="Username" onchange={onchange_password} />
                <Button label="Login" />
            </form>
        </div>
    }
}
