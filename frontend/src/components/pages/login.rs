use gloo::console::log;
use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::auth::login_form::{Data, LoginForm};
use crate::router::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let login_form_submit = Callback::from(|data: Data| {
        log!("Username is", data.username);
        log!("Password is", data.password);
    });

    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <div id={"form-container"}>
                <LoginForm form_title={"Login"} />
                <div class="sub-form">
                    <Link<Route> to={Route::Register}>{"No Account? Register Here"}</Link<Route>>
                </div>
            </div>
        </div>
    }
}
