use leptos::*;

use super::util::position_holder::PositionHolder;

struct ClientServerPositions {
    position_holder: PositionHolder,
}

impl ClientServerPositions {
    const SERVER: &'static str = "Server";
    const CLIENT: &'static str = "Client";
}

impl Default for ClientServerPositions {
    fn default() -> Self {
        Self {
            position_holder: PositionHolder::new(vec![
                String::from(ClientServerPositions::SERVER),
                String::from(ClientServerPositions::CLIENT),
            ]),
        }
    }
}

#[component]
pub fn ClientServerModeComponent() -> impl IntoView {
    view! {
        <div id="page-container">
        <div id="first" style="display:flex;flex-direction:column;
            flex:1;align-self:stretch;justify-self:stretch;
            padding:1px;border:solid green;">
        </div>

        <div id="second" style="flex:1;">
        <p> describe what is going on </p>
        </div>

        </div>
    }
}
