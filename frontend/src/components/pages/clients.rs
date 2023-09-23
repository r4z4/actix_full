use crate::components::{consults_display::ConsultsDisplay, consults_form::ConsultsForm, clients_form::ClientsForm, clients_display::ClientsDisplay};
use gloo::console::log;
use yew::prelude::*;

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
            <div id={"provider_display"}>
                <ClientsDisplay title={"Clients"} on_load={data_display_loaded} />
            </div>
        </div>
    }
}