use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::components::consults_display::ConsultsDisplay;

#[function_component(Consults)]
pub fn consults() -> Html {
    // let provider_form_submit = Callback::from(|data: Data| {
    //     log!("Name is", data.name);
    //     log!("Addr 1 is", data.address_1);
    //     log!("Addr 2 is", data.address_2);
    // });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div class={"entity-page"}>
            <h1>{"Consults"}</h1>
            <details>
                <summary>{"Add a Consult"}</summary>
                <div class={"form_container"}>
                    
                </div>
            </details>
            <div id={"provider_display"}>
                <ConsultsDisplay title={"Consults 👩"} on_load={data_display_loaded} />
            </div>
        </div>
    }
}