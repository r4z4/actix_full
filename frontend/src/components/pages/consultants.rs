use gloo::console::log;
use yew::prelude::*;

use crate::components::consultants::{consultants_display::ConsultantsDisplay, consultants_table::ConsultantsTable, consultants_form::ConsultantsForm};

#[function_component(Consultants)]
pub fn consultants() -> Html {
    // let provider_form_submit = Callback::from(|data: Data| {
    //     log!("Name is", data.name);
    //     log!("Addr 1 is", data.address_1);
    //     log!("Addr 2 is", data.address_2);
    // });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div class={"entity-page"}>
            <h1>{"Consultants Page"}</h1>
            <details>
                <summary>{"Add a Consultant"}</summary>
                <div class={"container"}>
                    <ConsultantsForm />
                </div>
            </details>
            <div id={"entity-table"}>
                // <ConsultantsDisplay title={"Consultants Display"} on_load={data_display_loaded} />
                <ConsultantsTable title={"Consultants Display"} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
