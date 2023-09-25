use crate::components::inputs::required_text_input::RequiredTextInput;
use crate::components::inputs::text_input::TextInput;
use crate::components::auth::matched_icon::MatchedIcon;
use crate::{components::button::Button, store::set_show_alert};
use crate::components::inputs::email_input::EmailInput;
use crate::store::{AuthStore, Store};
use reqwasm::http::Request;
use reqwasm::Error;
use serde_json::json;
// use crate::components::units::text_input::TextInput;
use std::ops::Deref;
use common::{ApiRegisterResponse, RegisterUserRequest};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
    pub onsubmit: Callback<RegisterUserRequest>,
}

pub async fn register_user(new_user: RegisterUserRequest) -> Result<ApiRegisterResponse, Error> {
    let body = json!(new_user);
    let response = Request::post("http://localhost:8000/register")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiRegisterResponse>()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

fn vec_to_html(list: &Vec<String>) -> Vec<Html> {
    list.iter()
        .map(|string| {
            html! {<ul class="errors-list">
                <li>{string.clone()}</li>
            </ul>}
        })
        .collect()
}

#[derive(Default, Clone)]
pub struct RePass {
    re_password: String,
    matched: Option<bool>,
}

#[function_component(RegisterForm)]
pub fn register_form(props: &Props) -> Html {
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    let (store, dispatch) = use_store::<Store>();
    let u_state: UseStateHandle<String> = use_state(|| String::from(""));
    let p_state: UseStateHandle<String> = use_state(|| String::from(""));
    let e_state: UseStateHandle<String> = use_state(|| String::from(""));
    let re_pass_state = use_state(|| RePass::default());
    let error_state = use_state(|| None);

    let u_state_clone = u_state.clone();
    let username_changed: Callback<String> = Callback::from(move |username| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<String> = u_state_clone.clone();
        cloned_state.set(username);
    });

    let p_state_clone = p_state.clone();
    let password_changed: Callback<String> = Callback::from(move |password| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<String> = p_state_clone.clone();
        cloned_state.set(password);
    });
    
    let p_state_clone = p_state.clone();
    let re_password_changed: Callback<String> = {
        let cloned_state: UseStateHandle<RePass> = re_pass_state.clone();
        Callback::from(move |re_password| {
            let mut data: RePass = cloned_state.deref().clone();
            if re_password == p_state_clone.deref().clone() {
                data.matched = Some(true);
            } else {
                data.matched = Some(false);
            }
            data.re_password = re_password;
            cloned_state.set(data);
        })
    };
    let re_pass_ref = re_pass_state.deref().clone();

    let e_state_clone = e_state.clone();
    let email_changed: Callback<String> = Callback::from(move |email| {
        let cloned_state: UseStateHandle<String> = e_state_clone.clone();
        cloned_state.set(email);
    });
    let error_state_clone = error_state.clone();
    let form_onsubmit = props.onsubmit.clone();
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        let dispatch_clone = dispatch.clone();
        let error_state_ref = error_state_clone.clone();
        event.prevent_default();

        // form_onsubmit.emit(data);
        if re_pass_ref.matched.is_some() && re_pass_ref.matched.unwrap() == false {
            let mut error_list = vec![];
            let mut errors = error_state_ref.clone();
            error_list.push("you suck".to_string());
            errors.set(Some(error_list));
        }
        let username = u_state.deref().clone();
        let password = p_state.deref().clone();
        let email = e_state.deref().clone();
        let new_user = RegisterUserRequest {
            username: username,
            password: password,
            email: email,
        };
        wasm_bindgen_futures::spawn_local(async move {
            let response = register_user(new_user).await;
            match response {
                Ok(response) => {
                    set_show_alert(format!("Congrats {}. You have registered successfully. Click here to log in!", response.username).to_string(), dispatch_clone.clone());
                    // dispatch.reduce_mut(|store| store.is_authenticated = true);
                    // navigator.push(&Route::Home);
                }
                Err(err) => {
                    let mut error_list = vec![];
                    let mut errors = error_state_ref.clone();
                    error_list.push(err.to_string());
                    errors.set(Some(error_list));
                    // navigator.push(&Route::Home);
                }
            }
            // Use this
            // log!(response.token)
        })
    });
    
    html! {
        <div>
            <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                if let Some(errors) = error_state.deref() {
                    <div class={"errors-div"}>
                        {vec_to_html(&errors)}
                    </div>
                }
                // <TextInput class={"half-input"} name="username" placeholder="Userame" handle_onchange={username_changed} />
                <RequiredTextInput input_type={"text"} name={"username"} class={"half-input"} placeholder={"Userame"} onchange={username_changed} />
                <RequiredTextInput input_type={"password"} name={"password"} class={"half-input"} placeholder={"Password"} onchange={password_changed} />
                <MatchedIcon state={re_pass_state.matched}/>
                <RequiredTextInput input_type={"password"} name={"re-password"} class={"half-input"} placeholder={"Re-enter Password"} onchange={re_password_changed} />
                <EmailInput name={"email"} class={"half-input"} placeholder={"Email"} handle_onchange={email_changed} required={true}/><br />
                // <TextInput name="password" placeholder="Password" handle_onchange={password_changed} />
                // <TextInput name="re_password" placeholder="Reenter Password" handle_onchange={re_password_changed} />
                <Button class={"submit-btn"} label={"Submit"} />
            </form>
        </div>
    }
}
