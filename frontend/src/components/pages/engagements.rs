use gloo::console::log;
use yew::prelude::*;

use crate::components::engagements::{engagement_form::EngagementForm, engagement_stats::EngagementStats, engagement_list::EngagementList};


#[function_component(Engagements)]
pub fn engagements() -> Html {
    // let provider_form_submit = Callback::from(|data: Data| {
    //     log!("Name is", data.name);
    //     log!("Addr 1 is", data.address_1);
    //     log!("Addr 2 is", data.address_2);
    // });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div class={"entity-page"}>
            <h1>{"Engagements"}</h1>
            <details>
                <summary>{"Add an Engagement"}</summary>
                <div class={"container"}>
                    <EngagementForm />
                </div>
            </details>
            <div class={"container"}>
                <EngagementStats />
                <EngagementList />
            </div>
        </div>
    }
}
