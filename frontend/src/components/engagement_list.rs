use yew::prelude::*;
use yewdux::prelude::*;

use super::engagement_item::EngagementItem;
use crate::store::Store;

#[function_component]
pub fn EngagementList() -> Html {
    let (store, _) = use_store::<Store>();
    let engagement_list = store.engagements.clone();

    html! {
        <div>
            <h4 class={"header-text white"}>{"Here is what other people had to say."}</h4>
            <div>
                {
                    engagement_list.into_iter().map(|engagement|{
                        let key = engagement.id.to_string();
                        html!{<EngagementItem {key} engagement={engagement.clone()} />}
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
