use common::RegisterUserRequest;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::auth::register_form::RegisterForm;
use crate::router::Route;

#[function_component(Register)]
pub fn register() -> Html {
    let register_form_submit = Callback::from(|data: RegisterUserRequest| {
        log!("Username is", data.username);
        log!("Password is", data.password);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <div id={"form-container"}>
                <RegisterForm form_title={"Register"} onsubmit={register_form_submit} />
                <div class="sub-form">
                    <Link<Route> to={Route::Login}>{"Already have an account? Login Here"}</Link<Route>>
                </div>
            </div>
        </div>
    }
}
