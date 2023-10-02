use std::ops::Deref;
use gloo_console::log;
use reqwasm::{Error, http::Request};
use serde_json::json;
use crate::{
    components::inputs::{select_input::SelectInput, date_input::DateInput, text_input::TextInput},
    store::{set_loading, set_show_alert, Store},
};
use common::{ClientPostRequest, ApiClientResponse};
use gloo::file::File;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

pub async fn post_client(new_client: ClientPostRequest) -> Result<ApiClientResponse, Error> {
    let body = json!(new_client);
    let response = Request::post("http://localhost:8000/api/clients/form")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiClientResponse>()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

#[function_component]
pub fn ClientsForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let file: UseStateHandle<Option<File>> = use_state(|| None);

    let client_id: UseStateHandle<Option<i32>> = use_state(|| None);
    let client_slug: UseStateHandle<Option<String>> = use_state(|| None);

    // let dob: UseStateHandle<Option<String>> = use_state(|| None);
    let client_f_name: UseStateHandle<Option<String>> = use_state(|| None);
    let client_l_name: UseStateHandle<Option<String>> = use_state(|| None);
    let client_company_name: UseStateHandle<Option<String>> = use_state(|| None);
    let client_dob = use_state(|| None);

    let client_address_one: UseStateHandle<String> = use_state(|| String::from(""));
    let client_address_two: UseStateHandle<Option<String>> = use_state(|| None);
    let client_city: UseStateHandle<String> = use_state(|| String::from(""));
    let client_state: UseStateHandle<String> = use_state(|| String::from(""));
    let client_zip: UseStateHandle<String> = use_state(|| String::from(""));

    let client_home_phone: UseStateHandle<String> = use_state(|| String::from(""));
    let client_mobile_phone: UseStateHandle<Option<String>> = use_state(|| None);
    let client_office_phone: UseStateHandle<Option<String>> = use_state(|| None);
    let client_email: UseStateHandle<String> = use_state(|| String::from(""));
    let account_id: UseStateHandle<i32> = use_state(|| 0);

    let min = use_state(|| 10);
    let message = use_state(|| Option::<String>::None);

    let text_input_ref = use_node_ref();

    let handle_required_input = {
        let email = client_email.clone();
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            message.set(None);
            email.set(value);
        })
    };

    let handle_l_name_input = {
        let l_name = client_l_name.clone();
        Callback::from(move |value| {
            l_name.set(value);
        })
    };

    let handle_f_name_input = {
        let f_name = client_f_name.clone();
        Callback::from(move |value| {
            f_name.set(value);
        })
    };

    let handle_company_name_input = {
        let company_name = client_company_name.clone();
        Callback::from(move |value| {
            company_name.set(value);
        })
    };

    let handle_dob_select = {
        let dob = client_dob.clone();
        Callback::from(move |value| {
            dob.set(Some(value));
        })
    };

    let handle_account_id_select = {
        let id = client_id.clone();
        Callback::from(move |value| {
            id.set(Some(value));
        })
    };


    let on_submit = {
        log!("on submit");
        let cloned_dispatch = dispatch.clone();

        let client_id = client_id.deref().clone();
        let client_f_name_deref = client_f_name.deref().clone();
        let client_l_name = client_l_name.deref().clone();
        let client_company_name = client_company_name.deref().clone();
        let client_dob = client_dob.deref().clone();

        let client_address_one = client_address_one.deref().clone();
        let client_address_two = client_address_two.deref().clone();
        let client_city = client_city.deref().clone();
        let client_state = client_state.deref().clone();
        let client_zip = client_zip.deref().clone();

        let client_home_phone = client_home_phone.deref().clone();
        let client_mobile_phone = client_mobile_phone.deref().clone();
        let client_office_phone = client_office_phone.deref().clone();
        let client_email = client_email.deref().clone();
        let account_id = account_id.deref().clone();

        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            log!("hitting callback");
            let dispatch = cloned_dispatch.clone();
            event.prevent_default();
            let client_f_name_new = client_f_name_deref.clone();
            let client_l_name_clone = client_l_name.clone();
            let client_company_name = client_company_name.clone();
            let client_address_one = client_address_one.clone();
            let client_address_two = client_address_two.clone();
            let client_city = client_city.clone();
            let client_state = client_state.clone();
            let client_zip = client_zip.clone();
            let client_home_phone = client_home_phone.clone();
            let client_mobile_phone = client_mobile_phone.clone();
            let client_office_phone = client_office_phone.clone();
            let client_email = client_email.clone();
            let account_id = account_id.clone();

            set_loading(true, dispatch.clone());

            if let Some(f_name) = client_f_name_new {
                if f_name.trim().len() < *min {
                    message.set(Some("First name must be at least 10 characters".to_string()));
                    set_loading(false, dispatch.clone());
                    return;
                }
            }

            if let Some(l_name) = client_l_name_clone {
                if &l_name.trim().len() < &min {
                    message.set(Some("Last name must be at least 10 characters".to_string()));
                    set_loading(false, dispatch.clone());
                    return;
                }
            }
            
            let new_client = ClientPostRequest {
                // client_id
                // client_slug 
                client_f_name: client_f_name_deref.clone(),
                client_l_name: client_l_name.clone(),
                client_company_name: client_company_name.clone(),
                client_dob: client_dob.clone(),
                client_address_one: client_address_one.clone(),
                client_address_two: client_address_two.clone(),
                client_city: client_city.clone(),
                client_state: client_state.clone(),
                client_zip: client_zip.clone(),
                client_home_phone: client_home_phone.clone(),
                client_mobile_phone: client_mobile_phone.clone(),
                client_office_phone: client_office_phone.clone(),
                client_email: client_email.clone(),
                account_id: account_id.clone(),
                // user_id: 
            };

            wasm_bindgen_futures::spawn_local(async move {
                log!("local spawned");
                let response = post_client(new_client).await;
                match response {
                    Ok(response) => {
                        // dispatch.reduce_mut(|store| store.token = Some(response.token));
                        // navigator.push(&Route::Consult);
                        set_show_alert(format!("Consult {} added successfully", response.client_id).to_string(), 1, dispatch.clone());
                    }
                    Err(err) => {
                        // let mut form_data = cloned_form_data.deref().clone();
                        // form_data.error = Some(err.to_string());
                        // cloned_form_data.set(data);
                        set_show_alert(format!("Error adding consult {}", err).to_string(), 1, dispatch.clone());
                        // navigator.push(&Route::Home);
                    }
                }
                // Use this
                // log!(response.token)
            });

            // Re-clone it
            let client_f_name_clone = client_f_name.clone();
            let dispatch = cloned_dispatch.clone();
            let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            text_input.set_value("");
            client_f_name.set(None);
            set_loading(false, dispatch);
        })
    };
    let client_dob_clone = client_dob.deref().clone();
    html! {
        <div class="form-container">
            <header class="form-header">
                <h2 class="header-text">{"Please rate your experience! We value your input."}</h2>
            </header>
            <form onsubmit={on_submit}>
                <div class="form-body">
                    <TextInput label={"First Name"} name={"client_f_name"} placeholder={"First Name"} onchange={handle_f_name_input} />
                    <TextInput label={"Last Name"} name={"client_l_name"} placeholder={"Last Name"} onchange={handle_l_name_input} />
                    <TextInput label={"Company Name"} name={"client_company_name"} placeholder={"Company Name"} onchange={handle_company_name_input} />

                    <DateInput date={client_dob_clone} label={"DOB"} onchange={handle_dob_select} />

                    <SelectInput label={"Account"} select_type={"account"} onchange={handle_account_id_select} />

                    <input
                        type="textarea"
                        ref={text_input_ref}
                        oninput={handle_required_input}
                        class="text-input"
                        placeholder="Consult notes ..."
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
