use crate::graphics::find_element_pos;

#[derive(Debug, Clone)]
pub(super) struct AllGoodScenario {}

impl AllGoodScenario {
    pub(super) fn execute(&self) {
        let server_pos = find_element_pos("Server");
        leptos::logging::log!("Server pos is {:?}", server_pos);
        let client_one_pos = find_element_pos("Client1");
        let client_two_pos = find_element_pos("Client2");
    }
}
