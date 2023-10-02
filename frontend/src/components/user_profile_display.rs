use common::parse_mime_type;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::{pages::user_profile::UserProfileData};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub user_data: UserProfileData,
    #[prop_or_default]
    pub onchange: Callback<u8>,
}


#[function_component]
pub fn UserProfileDisplay(props: &Props) -> Html {
    let user_data = &props.user_data;

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<u8>().unwrap();
        onchange.emit(selected)
    });

    html! {
        <div id={"user-profile-display"}>
            <div>
                <p><span class={"item-label"}>{"Username:"}</span>{user_data.username.clone()}</p>
            </div>
            <div>
                <p><span class={"item-label"}>{"Account ID:"}</span>{user_data.account_id}</p>
            </div>
            <div>
                <p><span class={"item-label"}>{"Email:"}</span>{user_data.email.clone()}</p>
            </div>
            <div>
                <p><span class={"item-label"}>{"User Since:"}</span>{user_data.created_at}</p>
            </div>
        </div>
    }
}