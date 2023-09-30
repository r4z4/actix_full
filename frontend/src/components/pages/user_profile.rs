use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::AuthStore;

#[function_component(UserProfile)]
pub fn user_profile() -> Html {
    let (store, dispatch) = use_store::<AuthStore>();

    html! {
        <div class={"entity-page"}>
            <h1>{format!("User Profile for {}", store.username.clone().unwrap())}</h1>

            <div class={"container"}>
                <p>{"Username"}</p>
                // <UserProfileTable />
            </div>
        </div>
    }
}
