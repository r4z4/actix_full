use std::{ops::Deref, borrow::Borrow};
use gloo_console::log;
use reqwasm::{Error, http::Request};
use serde_json::json;
use crate::{
    components::inputs::{select_input::SelectInput, date_input::DateInput, time_input::TimeInput, textarea_input::TextAreaInput},
    store::{set_loading, set_show_alert, Store},
};
use common::{ApiConsultResponse, ConsultPostRequest, ConsultPutRequest};
use gloo::file::File;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use super::{edit_modal::FormConsultEdit};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Option<FormConsultEdit>,
}


pub async fn post_consult(new_consult: ConsultPostRequest) -> Result<ApiConsultResponse, Error> {
    let body = json!(new_consult);
    let response = Request::post("http://localhost:8000/api/consults/form")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiConsultResponse>()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

pub async fn put_consult(new_consult: ConsultPutRequest) -> Result<ApiConsultResponse, Error> {
    let body = json!(new_consult);
    let response = Request::put(&format!("http://localhost:8000/api/consults/form/{}", new_consult.consult_id))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiConsultResponse>()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

#[derive(Debug)]
pub struct YewTempFile {
    /// The temporary file on disk.
    pub file: String,
    pub path: String,
    /// The value of the `content-type` header.
    pub content_type: Option<String>,
    /// The `filename` value in the `content-disposition` header.
    pub file_name: Option<String>,
    /// The size in bytes of the file.
    pub size: usize,
}

#[function_component]
pub fn ConsultsForm(props: &Props) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let file: UseStateHandle<Option<File>> = use_state(|| None);

    let header = if props.data.is_some() {"Edit Consult"} else {"Add Consult"};

    let data: UseStateHandle<Option<FormConsultEdit>> = use_state(|| props.data.clone());

    let consultant_id: UseStateHandle<Option<i32>> = use_state(|| None);
    let client_id: UseStateHandle<Option<i32>> = use_state(|| None);
    let location_id: UseStateHandle<Option<i32>> = use_state(|| None);

    let start_date: UseStateHandle<Option<String>> = use_state(|| None);
    let end_date: UseStateHandle<Option<String>> = use_state(|| None);

    let start_time: UseStateHandle<Option<String>> = use_state(|| None);
    let end_time: UseStateHandle<Option<String>> = use_state(|| None);

    let attachments: UseStateHandle<Option<Vec<i32>>> = use_state(|| None);

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

    let notes_handler = notes.clone();

    let handle_notes_input: Callback<String> = Callback::from(move |form_notes| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<Option<String>> = notes.clone();
        cloned_state.set(Some(form_notes));
    });

    let handle_client_id_select = {
        let id = client_id.clone();
        Callback::from(move |value| {
            id.set(Some(value));
        })
    };

    let handle_consultant_id_select = {
        let id = consultant_id.clone();
        Callback::from(move |value| {
            id.set(Some(value));
        })
    };

    let handle_location_id_select = {
        let id = location_id.clone();
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

    let handle_start_time_select = {
        let time = start_time.clone();
        Callback::from(move |value: String| {
            log!(value.clone());
            time.set(Some(value));
        })
    };

    let handle_end_date_select = {
        let date = end_date.clone();
        Callback::from(move |value| {
            date.set(Some(value));
        })
    };

    let handle_end_time_select = {
        let time = end_time.clone();
        Callback::from(move |value| {
            time.set(Some(value));
        })
    };

    
    let notes_h = notes_handler.clone();
    let on_submit = {
        log!("on submit");
        let cloned_dispatch = dispatch.clone();
        let consultant_id = consultant_id.deref().clone();
        let client_id = client_id.deref().clone();
        let location_id = location_id.deref().clone();

        let start_date = start_date.deref().clone();
        let end_date = end_date.deref().clone();
        let start_time = start_time.deref().clone();
        let end_time = end_time.deref().clone();
        let notes = notes_handler.deref().clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            log!("hitting callback");
            let dispatch = cloned_dispatch.clone();
            event.prevent_default();
            let notes = notes.clone();
            let start_date_ta = start_date.clone();
            let end_date_ta = end_date.clone();
            let start_time_ta = start_time.clone();
            let end_time_ta = end_time.clone();
            let notes_ta = notes.clone();
            set_loading(true, dispatch.clone());

            if let Some(nts) = notes_ta {
                if nts.trim().len() < *min {
                    message.set(Some("Text must be at least 10 characters".to_string()));
                    set_loading(false, dispatch.clone());
                    return;
                }
            }
            
            if let Some(consult) = data.deref() {
                let edit_consult = ConsultPutRequest {
                    consult_id: client_id.unwrap(),
                    client_id: client_id.unwrap(),
                    consultant_id: consultant_id.unwrap(),
                    location_id: location_id.unwrap(),
                    start_date: start_date_ta,
                    end_date: end_date_ta,
                    start_time: start_time_ta,
                    end_time: end_time_ta,
                    notes: notes.clone(),
                };
                wasm_bindgen_futures::spawn_local(async move {
                    log!("local spawned");
                    let response = put_consult(edit_consult).await;
                    match response {
                        Ok(response) => {
                            // dispatch.reduce_mut(|store| store.token = Some(response.token));
                            // navigator.push(&Route::Consult);
                            set_show_alert(format!("Consult {} updated successfully", response.consult_id).to_string(), 1, dispatch.clone());
                        }
                        Err(err) => {
                            // let mut form_data = cloned_form_data.deref().clone();
                            // form_data.error = Some(err.to_string());
                            // cloned_form_data.set(data);
                            log!("Logging => {}", err.to_string());
                            set_show_alert(format!("Error updating consult {}", err).to_string(), 2, dispatch.clone());
                            // navigator.push(&Route::Home);
                        }
                    }
                    // Use this
                    // log!(response.token)
                })
            } else {
                wasm_bindgen_futures::spawn_local(async move {
                    let new_consult = ConsultPostRequest {
                        // consult_id: i32,
                        client_id: client_id.unwrap(),
                        consultant_id: consultant_id.unwrap(),
                        location_id: location_id.unwrap(),
                        start_date: start_date_ta,
                        end_date: end_date_ta,
                        start_time: start_time_ta,
                        end_time: end_time_ta,
                        notes: notes.clone(),
                    };
                    log!("local spawned");
                    let response = post_consult(new_consult).await;
                    match response {
                        Ok(response) => {
                            // dispatch.reduce_mut(|store| store.token = Some(response.token));
                            // navigator.push(&Route::Consult);
                            set_show_alert(format!("Consult {} added successfully", response.consult_id).to_string(), 1, dispatch.clone());
                        }
                        Err(err) => {
                            // let mut form_data = cloned_form_data.deref().clone();
                            // form_data.error = Some(err.to_string());
                            // cloned_form_data.set(data);
                            log!("Logging => {}", err.to_string());
                            set_show_alert(format!("Error adding consult {}", err).to_string(), 2, dispatch.clone());
                            // navigator.push(&Route::Home);
                        }
                    }
                    // Use this
                    // log!(response.token)
                })
            }
            // Re-clone it
            let dispatch = cloned_dispatch.clone();
            let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            text_input.set_value("");
            notes_handler.set(None);
            set_loading(false, dispatch);
        })
    };
    let final_start_date_clone = start_date.clone().deref().clone();
    let final_end_date_clone = end_date.clone().deref().clone();
    let final_start_time_clone = start_time.clone().deref().clone();
    let final_end_time_clone = end_time.clone().deref().clone();
    let notes_h = notes_h.clone();
    html! {
        <div class="form-container">
            <header class="form-header">
                <h2 class="header-text">{header}</h2>
            </header>
            <form onsubmit={on_submit} enctype={"multipart/form-data"}>
                <div class="form-body">
                    <SelectInput required={true} label={"Location"} select_type={"location"} onchange={handle_location_id_select} />
                    <SelectInput required={true} label={"Consultant"} select_type={"consultant"} onchange={handle_consultant_id_select} />
                    <SelectInput required={true} label={"Client"} select_type={"client"} onchange={handle_client_id_select} />

                    <DateInput required={true} date={final_start_date_clone} label={"Start Date"} onchange={handle_start_date_select} />
                    <DateInput date={final_end_date_clone} label={"End Date"} onchange={handle_end_date_select} />

                    <TimeInput time={final_start_time_clone} label={"Start Time"} onchange={handle_start_time_select} />
                    <TimeInput time={final_end_time_clone} label={"End Time"} onchange={handle_end_time_select} />

                    <TextAreaInput name={"notes"} label={"Notes"} class={"text-input"} value={notes_h.deref().clone()} placeholder={"Consult notes ..."} onchange={handle_notes_input} />

                    // <input
                    //     type="textarea"
                    //     ref={text_input_ref}
                    //     oninput={handle_input}
                    //     class="text-input"
                    //     placeholder="Consult notes ..."
                    // />
                // FIXME: Convert accept from PHTMap of accepted mime_types
                <input type={"file"} name={"file"} accept={"image/*,audio/*,video/*"} multiple={true} />
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
                // <form action="/upload" method="post" enctype="multipart/form-data">
                //     <input type={"file"} name={"file"} />
                //     <input type={"submit"} />
                // </form>
            </form>
        </div>
    }
}
