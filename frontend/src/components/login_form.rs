use common::ApiLoginResponse;
use reqwasm::http::Request;
use reqwasm::Error;
use serde_json::json;
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::functional::use_store;

use crate::components::button::Button;
use crate::router::{switch, Route};
use crate::store::AuthStore;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
    pub error: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
}

pub async fn login_user(username: String, password: String) -> Result<ApiLoginResponse, Error> {
    let body = json!({
        "username": username,
        "password": password
    });
    let response = Request::post("http://localhost:8000/auth")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiLoginResponse>()
        .await;

    response

    // match response {
    //     Ok(response) => response.unwrap(),
    //     Err(err) => Error::to_string(&self);
    // }
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
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    let navigator = use_navigator().unwrap();

    let onchange_username = {
        let cloned_data_state = state.clone();
        let dispatch = auth_dispatch.clone();
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
        let dispatch = auth_dispatch.clone();
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
        let cloned_data_state = state.clone();
        let dispatch = auth_dispatch.clone();
        let navigator = navigator.clone();
        event.prevent_default();
        let username = state.username.clone();
        let password = state.password.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = login_user(username, password).await;
            match response {
                Ok(response) => {
                    dispatch.reduce_mut(|store| store.token = Some(response.token));
                    dispatch.reduce_mut(|store| store.is_authenticated = true);
                    navigator.push(&Route::Home);
                }
                Err(err) => {
                    let mut data = cloned_data_state.deref().clone();
                    data.error = Some(err.to_string());
                    cloned_data_state.set(data);
                    // navigator.push(&Route::Home);
                }
            }
            // Use this
            // log!(response.token)
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
                if cloned_state.error.is_some() {
                    <p>{cloned_state.error.as_ref()}</p>
                }
                <input type="text" placeholder="Username" onchange={onchange_username} /><br />
                <input type="text" placeholder="Password" onchange={onchange_password} /><br />
                <Button label="Login" />
            </form>
        </div>
    }
}
