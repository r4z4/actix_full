use gloo::console::log;
use yew::prelude::*;

use crate::components::consults::{consults_form::ConsultsForm, consults_display::ConsultsDisplay, consults_table::ConsultsTable};

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
                <div class={"container"}>
                    <ConsultsForm />
                </div>
            </details>
            <div id={"entity-table"}>
                // <ConsultsDisplay title={"Consults"} on_load={data_display_loaded} />
                <ConsultsTable title={"Consultants Display"} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
