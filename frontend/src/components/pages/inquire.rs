use gloo::console::log;
use yew::prelude::*;

#[function_component(Inquire)]
pub fn inquire() -> Html {
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

                </div>
            </details>
            <div class={"container"}>

            </div>
        </div>
    }
}
