use crate::components::button::Button;
use crate::components::email_input::EmailInput;
use crate::store::AuthStore;
// use crate::components::units::text_input::TextInput;
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
    pub re_password: String,
    pub email: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
    pub onsubmit: Callback<Data>,
}

#[function_component(RegisterForm)]
pub fn register_form(props: &Props) -> Html {
    let (store, dispatch) = use_store::<AuthStore>();
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    let navigator = use_navigator().unwrap();

    let cloned_state: UseStateHandle<Data> = state.clone();
    // let username_changed: Callback<String> = Callback::from(move |username| {
    //     let mut data: Data = cloned_state.deref().clone();
    //     data.username = username;
    //     cloned_state.set(data);
    // });
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

    let cloned_state: UseStateHandle<Data> = state.clone();
    let password_changed: Callback<String> = Callback::from(move |password| {
        let mut data: Data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let re_password_changed: Callback<String> = Callback::from(move |re_password| {
        let mut data: Data = cloned_state.deref().clone();
        data.re_password = re_password;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let email_changed: Callback<String> = Callback::from(move |password| {
        let mut data: Data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });
    html! {
        <div>
            <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                // <TextInput class={"half-input"} name="username" placeholder="Userame" handle_onchange={username_changed} />
                <input type="text" class={"half-input"} name="username" placeholder="Userame" onchange={onchange_username} /><br />
                <EmailInput name="email" placeholder="Email" handle_onchange={email_changed} /><br />
                // <TextInput name="password" placeholder="Password" handle_onchange={password_changed} />
                // <TextInput name="re_password" placeholder="Reenter Password" handle_onchange={re_password_changed} />
                <Button label="Submit" />
            </form>
        </div>
    }
}
