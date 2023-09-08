use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::{delete_engagement, set_show_alert, Store};
use common::Engagement;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub engagement: Engagement,
}

fn confirm_delete(message: &str) -> bool {
    web_sys::Window::confirm_with_message(&web_sys::window().unwrap(), message).unwrap()
}

#[function_component]
pub fn EngagementItem(props: &Props) -> Html {
    let (_, dispatch) = use_store::<Store>();

    let on_delete = {
        let cloned_dispatch = dispatch.clone();
        let engagement_id = props.engagement.id.clone();
        Callback::from(move |_: MouseEvent| {
            let dispatch = cloned_dispatch.clone();
            let confirmed = confirm_delete("Are you sure?");

            if confirmed {
                delete_engagement(engagement_id, dispatch.clone());
                set_show_alert("Engagement deleted successfully".to_string(), dispatch);
            }
        })
    };

    html! {
        <div class="item-container">
            <div class="rating-block">
                {props.engagement.rating}
            </div>
            <button class="rating-button" onclick={on_delete}>{"X"}</button>
            <p>
                {&props.engagement.text}
            </p>
        </div>
    }
}
