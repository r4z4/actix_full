mod api;
mod components;
mod store;

use components::{
    alert::{AlertComponent, Props as AlertProps},
    engagement_form::EngagementForm,
    engagement_list::EngagementList,
    engagement_stats::EngagementStats,
};
use store::Store;
use stylist::Style;
use yew::prelude::*;
use yewdux::prelude::*;

const CSS_FILE: &str = include_str!("main.css");

#[function_component]
fn App() -> Html {
    let stylesheet = Style::new(CSS_FILE).unwrap();
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <>
            if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
            <main class="main-class">
                <div class={stylesheet}> 
                    <EngagementForm />
                    <EngagementStats />
                    <EngagementList />
                </div>
            </main>
            if *loading {
                <div
                    class="loading-anim"
                    role="status">
                    <span
                    class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
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