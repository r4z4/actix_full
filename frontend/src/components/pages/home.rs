use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::components::nav::Nav;
use crate::router::{switch, Route};
use crate::store::AuthStore;

#[function_component(Home)]
pub fn home() -> Html {
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    // let state: UseStateHandle<Data> = use_state(|| Data::default());
    // let navigator = use_navigator().unwrap();

    html! {
        <div id="home_main">
            <h1>{"Home"}</h1>
            if auth_store.token.is_some() {
                <h1>{"Some Office"}</h1>
                <p>{"Very privileged data lives here that only the logged in can see!"}</p>
            } else {
                <div>
                    <h1>{"Please Login to view our offerings."}</h1>
                    <Link<Route> to={Route::Register}>{"Login"}</Link<Route>>

                </div>
            }
        </div>
    }
}
