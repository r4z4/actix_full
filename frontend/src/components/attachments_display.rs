use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::consults::view_modal::AttachmentData;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub attachments_data: Vec<AttachmentData>,
    #[prop_or_default]
    pub onchange: Callback<u8>,
}

#[function_component]
pub fn AttachmentsDisplay(props: &Props) -> Html {
    let attachments_data = &props.attachments_data;

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<u8>().unwrap();
        onchange.emit(selected)
    });

    html! {
        <ul class="list-none flex items-center justify-around my-7">
            { for (1..=10).map(|i| {
                let label = i.to_string();
                let id = format!("num{}", i);

                html! {
                    <li>

                    </li>
                }
            })}
        </ul>
    }
}