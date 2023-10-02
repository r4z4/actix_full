mod api;
mod components;
mod router;
mod store;

use components::alert::{AlertComponent, Props as AlertProps};
use store::Store;
use stylist::Style;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch, prelude::Link};
use yewdux::prelude::*;

use crate::{
    components::nav::Nav,
    router::{switch, Route}, store::AuthStore,
};

const CSS_FILE: &str = include_str!("main.css");

#[function_component]
fn App() -> Html {
    let stylesheet = Style::new(CSS_FILE).unwrap();
    let (auth_store, _) = use_store::<AuthStore>();
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let typ = store.alert_input.alert_typ.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let alert_props = AlertProps {
        message,
        typ,
        delay_ms: 5000,
    };

    let auth_ref = auth_store.as_ref();

    html! {
        <>
            <main class={stylesheet}>
            if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        typ={alert_props.typ}
                        delay_ms={alert_props.delay_ms}
                     />
                }

            if auth_ref.token.is_some() {
                <BrowserRouter>
                    // Nav needs to be child of BrowserRouter
                    // <Logout label={"⇥"} />
                    <Nav color={"black"} />
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            } else {
                <BrowserRouter>
                    <h2 class={"login-route"}><Link<Route> to={Route::Login}>{"Welcome to the External Review Portal. Click the Key to Login & Continue. 🔑"}</Link<Route>></h2>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            }

            </main>
            if *loading {
                <div
                    class="loading-anim"
                    role="status">
                    <span
                    class="loading-span"
                    >{"Loading..."}</span
                >
                </div>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
