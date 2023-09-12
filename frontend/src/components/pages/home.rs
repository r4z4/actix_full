use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::store::AuthStore;
use crate::components::nav::Nav;
use crate::router::{switch, Route};

#[function_component(Home)]
pub fn home() -> Html {
    let (store, dispatch) = use_store::<AuthStore>();
    // let state: UseStateHandle<Data> = use_state(|| Data::default());
    // let navigator = use_navigator().unwrap();

    html! {
        <div>
            <h1>{"Home"}</h1>
            if store.token.is_some() {
                <h1>{"Halo"}</h1>
            } else {
                <h1>{"Goodbye"}</h1>
            }
        </div>
    }
}