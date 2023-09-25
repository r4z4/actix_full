use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[function_component(HomeCard)]
pub fn home_card(props: &Props) -> Html {
    html! {
        <div class={"home-card-main"}>
        <div class={"home-card-relative"}>
          <div class={"home-card-outline"}>
            <div class={"card-header"}>{"Pay as you go"}</div>
            <div class={"card-align"}>
              <div class={"euro-center"}>
                <div class={"euro-subcenter"}>
                  <span class={"pay-text"}>{"1.4%"}</span>
                  <span class={"pay-text"}>{"+"}</span>
                  <span class={"pay-text"}>{"20p"}</span>
                </div>
                <span class={"euro-text"}>{"for European cards"}</span>
              </div>
              <div class={"pay-div"}>
                <div class={"div-list"}>
                  <span class={"pay-text"}>{"2.9%"}</span>
                  <span class={"pay-text"}>{"+"}</span>
                  <span class={"pay-text"}>{"20p"}</span>
                </div>
                <span class={"euro-text"}>{"for non-European cards"}</span>
              </div>
            </div>
            <div class={"c1-list"}>
              <ul>
                <li class={"flex items-center"}>
                  <div class={"green-bg"}>
                    <svg class={"svg-ico"} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class={"icon-umbrella"}><path class={"primary"} d="M11 3.05V2a1 1 0 0 1 2 0v1.05A10 10 0 0 1 22 13c0 1.33-2 1.33-2 0a2 2 0 1 0-4 0c0 1.33-2 1.33-2 0a2 2 0 1 0-4 0c0 1.33-2 1.33-2 0a2 2 0 1 0-4 0c0 1.33-2 1.33-2 0a10 10 0 0 1 9-9.95z"/><path class={"secondary"} d="M11 14a1 1 0 0 1 2 0v5a3 3 0 0 1-6 0 1 1 0 0 1 2 0 1 1 0 0 0 2 0v-5z"/></svg>
                  </div>
                  <span class={"pay-text"}>{"No setup, monthly, or hidden fees"}</span>
                </li>
                <li class={"flex items-center mt-3"}>
                  <div class={"green-bg"}>
                    <svg class={"svg-ico"} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class={"icon-shopping-bag"}><path class={"primary"} d="M5 8h14a1 1 0 0 1 1 .92l1 12A1 1 0 0 1 20 22H4a1 1 0 0 1-1-1.08l1-12A1 1 0 0 1 5 8z"/><path class={"secondary"} d="M9 10a1 1 0 0 1-2 0V7a5 5 0 1 1 10 0v3a1 1 0 0 1-2 0V7a3 3 0 0 0-6 0v3z"/></svg>
                  </div>
                  <span class={"pay-text"}>{"Pay only for what you use"}</span>
                </li>
                <li class={"flex items-center mt-3"}>
                  <div class={"green-bg"}>
                    <svg class={"svg-ico"} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class={"icon-pie-chart"}><path class={"primary"} d="M14 13h6.78a1 1 0 0 1 .97 1.22A10 10 0 1 1 9.78 2.25a1 1 0 0 1 1.22.97V10a3 3 0 0 0 3 3z"/><path class={"secondary"} d="M20.78 11H14a1 1 0 0 1-1-1V3.22a1 1 0 0 1 1.22-.97c3.74.85 6.68 3.79 7.53 7.53a1 1 0 0 1-.97 1.22z"/></svg>
                  </div>
                  <span class={"pay-text"}>{"Real-time fee reporting"}</span>
                </li>
              </ul>
            </div>
            <a class={"block-link"} href="#">
              <span>{"Create account"}</span>
              <span class={"font-medium text-gray-700 ml-2"}>{"➔"}</span>
            </a>
          </div>
          <div class={"card-two-div"}>
            <div class={"card-two-inner"}>
              <div class={"c2-header"}>{"Enterprise"}</div>
              <div class={"c2-subhead"}>
                {"Stripe offers everything needed to run an online business at scale. Get in touch for details."}
              </div>
              <div class={"flex-div"}>
                <div class={"card-tile"}>{"Account management"}</div>
                <div class={"card-tile"}>{"Volume discounts"}</div>
                <div class={"card-tile"}>{"Migration assistance"}</div>
                <div class={"card-tile"}>{"Dedicated support"}</div>
              </div>
              <a class={"block-link"} href="#">
                <span>{"Contact sales"}</span>
                <span class={"pay-text"}>{"➔"}</span>
              </a>
            </div>
          </div>
        </div>
      </div>
    }
}