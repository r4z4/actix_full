use std::ops::Deref;

use crate::store::{set_show_alert, set_token, AuthStore, Store};
use common::{ApiLoginResponse, ErrorResponse};
use reqwasm::http;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback, Html, UseStateHandle};
use yewdux::functional::use_store;

pub async fn api_refresh_access_token(token: String) -> Result<ApiLoginResponse, String> {
    let response = match http::Request::get("http://localhost:8000/api/auth/refresh")
        .header("Content-Type", "application/json")
        .header("Authorization", &token)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<ApiLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

fn confirm_refresh(message: &str) -> bool {
    web_sys::Window::confirm_with_message(&web_sys::window().unwrap(), message).unwrap()
}

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
    pub error: Option<String>,
}

#[function_component(RefreshToken)]
pub fn refresh_token() -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    let (store, dispatch) = use_store::<Store>();

    let on_refresh = {
        let cloned_auth_dispatch = auth_dispatch.clone();
        let cloned_state = state.clone();
        let current_token = auth_store.token.clone();
        Callback::from(move |_: MouseEvent| {
            let current_token = current_token.clone();
            let cloned_state = cloned_state.clone();
            let auth_dispatch = cloned_auth_dispatch.clone();
            let dispatch = dispatch.clone();
            let confirmed = confirm_refresh("Are you sure?");

            if confirmed {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = api_refresh_access_token(current_token.clone().unwrap()).await;
                    match response {
                        Ok(response) => {
                            let token = response.token.clone();
                            auth_dispatch.reduce_mut(|store| store.token = Some(response.token));
                            set_token(token, auth_dispatch.clone());
                            set_show_alert(
                                "Token has been refreshed. Thank you!".to_string(),
                                1,
                                dispatch,
                            );
                        }
                        Err(err) => {
                            let cloned_state = cloned_state.clone();
                            let mut data = cloned_state.deref().clone();
                            data.error = Some(err.to_string());
                            cloned_state.set(data);
                            // navigator.push(&Route::Home);
                        }
                    }
                    // Use this
                    // log!(response.token)
                })
            }
        })
    };

    html! {
        <div>
            <h3>{"Refresh Token"}</h3>
            if state.error.is_some() {
                <p>{state.error.clone()}</p>
            }
            <button class="rating-button" onclick={on_refresh}>{"🗘"}</button>
        </div>
    }
}
