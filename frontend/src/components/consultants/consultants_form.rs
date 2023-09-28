use std::ops::Deref;
use gloo_console::log;
use reqwasm::{Error, http::Request};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    components::inputs::{select_input::SelectInput, date_input::DateInput, required_text_input::RequiredTextInput},
    store::{set_loading, set_show_alert, Store},
};
use gloo::file::File;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiConsultantResponse {
    pub consultant_id: i32,
    pub consultant_start: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsultantPostRequest {
    pub consultant_f_name: String,
    pub consultant_l_name: String,
    pub specialty_id: i32,
    pub territory_id: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub notes: Option<String>,
}

pub async fn post_consultant(new_consultant: ConsultantPostRequest) -> Result<ApiConsultantResponse, Error> {
    let body = json!(new_consultant);
    let response = Request::post("http://localhost:8000/api/consultants/form")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiConsultantResponse>()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

#[function_component]
pub fn ConsultsForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let file: UseStateHandle<Option<File>> = use_state(|| None);
    let f_state: UseStateHandle<String> = use_state(|| String::from(""));
    let l_state: UseStateHandle<String> = use_state(|| String::from(""));

    let error_state = use_state(|| None);

    let f_state_clone = f_state.clone();
    let consultant_f_name_changed: Callback<String> = Callback::from(move |consultant_f_name| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<String> = f_state_clone.clone();
        cloned_state.set(consultant_f_name);
    });

    let l_state_clone = l_state.clone();
    let consultant_l_name_changed: Callback<String> = Callback::from(move |consultant_l_name| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<String> = l_state_clone.clone();
        cloned_state.set(consultant_l_name);
    });

    let start_date: UseStateHandle<Option<String>> = use_state(|| None);
    let end_date: UseStateHandle<Option<String>> = use_state(|| None);
    let specialty_id: UseStateHandle<Option<i32>> = use_state(|| None);
    let territory_id: UseStateHandle<Option<i32>> = use_state(|| None);

    let consultant_f_name: UseStateHandle<Option<String>> = use_state(|| None);
    let consultant_l_name: UseStateHandle<Option<String>> = use_state(|| None);

    let notes = use_state(|| None);

    let min = use_state(|| 10);
    let message = use_state(|| Option::<String>::None);

    let text_input_ref = use_node_ref();

    // let handle_select = {
    //     let rating = rating.clone();
    //     Callback::from(move |value| {
    //         rating.set(value);
    //     })
    // };

    // let handle_file = {
    //     let file = file.clone();
    //     dbg!(&file);
    //     Callback::from(move |value| {
    //         file.set(value);
    //     })
    // };

    let handle_input = {
        let notes = notes.clone();
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            message.set(None);
            notes.set(Some(value));
        })
    };

    let handle_territory_id_select = {
        let id = territory_id.clone();
        Callback::from(move |value| {
            id.set(Some(value));
        })
    };

    let handle_specialty_id_select = {
        let id = specialty_id.clone();
        Callback::from(move |value| {
            id.set(Some(value));
        })
    };

    let handle_start_date_select = {
        let date = start_date.clone();
        Callback::from(move |value| {
            date.set(Some(value));
        })
    };

    let handle_end_date_select = {
        let date = end_date.clone();
        Callback::from(move |value| {
            date.set(Some(value));
        })
    };
    let error_state_clone = error_state.clone();
    let on_submit = {
        log!("on submit");
        let cloned_dispatch = dispatch.clone();

        let specialty_id = specialty_id.deref().clone();
        let territory_id = territory_id.deref().clone();

        let start_date = start_date.deref().clone();
        let end_date = end_date.deref().clone();

        let notes = notes.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            log!("hitting callback");
            let dispatch = cloned_dispatch.clone();
            let consultant_f_name = consultant_f_name.deref();
            let consultant_l_name = consultant_l_name.deref();
            let error_state_ref = error_state_clone.clone();
            event.prevent_default();
            let notes = notes.clone();
            let start_date_ta = start_date.clone();
            let end_date_ta = end_date.clone();

            let binding = notes.clone();
            let notes_ta = binding.deref();
            set_loading(true, dispatch.clone());

            if let Some(notes) = notes_ta {
                if notes.trim().len() < *min {
                    message.set(Some("Text must be at least 10 characters".to_string()));
                    set_loading(false, dispatch.clone());
                    return;
                }
            }

            let new_consultant = ConsultantPostRequest {
                // consult_id: i32,

                consultant_f_name: consultant_f_name.clone().unwrap(),
                consultant_l_name: consultant_l_name.clone().unwrap(),
                specialty_id: specialty_id.unwrap(),
                territory_id: specialty_id.unwrap(),
                start_date: start_date_ta,
                end_date: end_date_ta,

                notes: notes_ta.clone(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                log!("local spawned");
                let response = post_consultant(new_consultant).await;
                match response {
                    Ok(response) => {
                        // dispatch.reduce_mut(|store| store.token = Some(response.token));
                        // navigator.push(&Route::Consult);
                        set_show_alert(format!("Consult {} added successfully", response.consultant_id).to_string(), dispatch.clone());
                    }
                    Err(err) => {
                        set_show_alert(format!("Error adding consultant {}", err).to_string(), dispatch.clone());
                        let mut error_list = vec![];
                        let mut errors = error_state_ref.clone();
                        error_list.push(err.to_string());
                        errors.set(Some(error_list));
                    }
                }
                // Use this
                // log!(response.token)
            });

            // Re-clone it
            let dispatch = cloned_dispatch.clone();
            let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            text_input.set_value("");
            notes.set(None);
            set_loading(false, dispatch);
        })
    };
    let final_start_date_clone = start_date.clone().deref().clone();
    let final_end_date_clone = end_date.clone().deref().clone();

    html! {
        <div class="form-container">
            <header class="form-header">
                <h2 class="header-text">{"Please rate your experience! We value your input."}</h2>
            </header>
            <form onsubmit={on_submit}>
                <div class="form-body">

                    <RequiredTextInput input_type={"text"} name={"consultant_f_name"} class={"half-input"} placeholder={"First Name"} onchange={consultant_f_name_changed} />
                    <RequiredTextInput input_type={"text"} name={"consultant_l_name"} class={"half-input"} placeholder={"Userame"} onchange={consultant_l_name_changed} />

                    <SelectInput label={"Specialty"} select_type={"location"} onchange={handle_specialty_id_select} />
                    <SelectInput label={"Terrotiry"} select_type={"consultant"} onchange={handle_territory_id_select} />

                    <DateInput date={final_start_date_clone} label={"Start Date"} onchange={handle_start_date_select} />
                    <DateInput date={final_end_date_clone} label={"End Date"} onchange={handle_end_date_select} />

                    <input
                        type="textarea"
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="text-input"
                        placeholder="Consultant notes ..."
                    />
                <button
                    type="submit"
                    class={format!(
                        "submit-button {}",
                        if *loading { "plus-load"} else {"no-load"}
                    )}
                >
                    {"Submit"}
                </button>
                </div>
                {if let Some(msg) = message.as_ref() {
                    html! { <div class="message">{msg.clone()}</div> }
                } else {
                    html! {}
                }}
                <input type="file" />
            </form>
        </div>
    }
}