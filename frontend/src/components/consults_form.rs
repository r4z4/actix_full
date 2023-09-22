use std::ops::Deref;

use super::rating::Rating;
use gloo_console::log;
use crate::{
    components::{select_input::SelectInput, date_input::DateInput, time_input::TimeInput},
    store::{set_engagement, set_loading, set_show_alert, Store},
};
use common::Engagement;
use gloo::file::File;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn ConsultsForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let text = use_state(String::new);
    let file: UseStateHandle<Option<File>> = use_state(|| None);
    let start_date: UseStateHandle<Option<String>> = use_state(|| None);
    let end_date: UseStateHandle<Option<String>> = use_state(|| None);

    let start_time: UseStateHandle<String> = use_state(|| "00:00".to_owned());
    let end_time: UseStateHandle<Option<String>> = use_state(|| None);
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
        let text = text.clone();
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            message.set(None);
            text.set(value);
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
            time.set(value);
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

    let on_submit = {
        let cloned_dispatch = dispatch.clone();
        let text = text.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let dispatch = cloned_dispatch.clone();
            event.prevent_default();
            set_loading(true, dispatch.clone());

            if text.trim().len() < *min {
                message.set(Some("Text must be at least 10 characters".to_string()));
                set_loading(false, dispatch.clone());
                return;
            }

            // let new_engagement = Engagement {
            //     id: Uuid::new_v4(),
            //     text: text.to_string(),
            //     rating: *rating,
            // };

            // set_engagement(new_engagement, dispatch.clone());
            // set_show_alert("Feeback added successfully".to_string(), dispatch.clone());

            // let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            // text_input.set_value("");
            // text.set(String::new());
            // set_loading(false, dispatch);
        })
    };
    let final_start_time_clone = start_time.clone().deref().clone();
    html! {
        <div class="form-container">
            <header class="form-header">
                <h2 class="header-text">{"Please rate your experience! We value your input."}</h2>
            </header>
            <form onsubmit={on_submit}>
                <div class="form-body">
                    <SelectInput select_type={"location"} />
                    <SelectInput select_type={"consultant"} />
                    <SelectInput select_type={"client"} />
                    <DateInput label={"Start Date"} onchange={handle_start_date_select} />
                    <DateInput label={"End Date"} onchange={handle_end_date_select} />
                    <TimeInput time={final_start_time_clone} label={"Start Time"} onchange={handle_start_time_select} />
                    <input
                        type="text"
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="text-input"
                        placeholder="Please tell us more ..."
                    />
                <button
                    type="submit"
                    class={format!(
                        "submit-button {}",
                        if *loading { "plus-load"} else {"no-load"}
                    )}
                >
                    {"Send"}
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
