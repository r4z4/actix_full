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

// fn attachment_to_html(attachment: AttachmentData) -> Html {
    
// }

fn vec_to_html(list: &Vec<AttachmentData>) -> Vec<Html> {
    list.iter()
        .map(|attachment| {
            html! {
            <div class="media-display">
                if attachment.typ == "img/png".to_owned() {
                    <img src={attachment.url.clone()} />
                }
                if attachment.typ == "img/jpg".to_owned() {
                    <img src={attachment.url.clone()} />
                }
                if attachment.typ == "audio/wav".to_owned() {
                    <audio controls={true}>
                        <source src={attachment.url.clone()} type="audio/wav" />
                        {"Your browser does not support the audio element."}
                    </audio>
                }
                if attachment.typ == "video/webm".to_owned() {
                    <video width="320" height="240" controls={true} >
                        <source src={attachment.url.clone()} type="video/webm" />
                        // Use with multiple types
                        // <source src="movie.ogg" type="video/ogg">
                        {"Your browser does not support the video tag."}
                    </video>
                }
            </div>
        }
        })
        .collect()
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
        <div>
            {vec_to_html(attachments_data)}
        </div>
    }
}