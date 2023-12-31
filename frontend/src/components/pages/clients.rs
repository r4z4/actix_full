use gloo::console::log;
use yew::prelude::*;

use crate::components::clients::{clients_table::ClientsTable, clients_form::ClientsForm};

#[function_component(Clients)]
pub fn clients() -> Html {
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div class={"entity-page"}>
            <h1>{"Clients"}</h1>
            <details>
                <summary>{"Add a Client"}</summary>
                <div class={"container"}>
                    <ClientsForm />
                </div>
            </details>
            <div id={"entity-table"}>
                <ClientsTable title={"Clients"} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
