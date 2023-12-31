use std::collections::BTreeMap;

use leptos::*;

#[component]
pub(super) fn InitialSetupComponent(
    server_count: ReadSignal<u32>,
    setup: Signal<BTreeMap<u32, (Vec<u32>, Vec<u32>)>>,
) -> impl IntoView {
    view! {
        <table style="padding:1em;width:100%;">
            <tr>
            <th>Server</th>
            <th>Initail SetUp</th>
            {move || if server_count() != 3 { Some(view! {<th>With {server_count()} servers</th>})} else {None}}
            </tr>
            {
                move || setup().into_iter().map(|(server_num, values)| view! {
                    <tr>
                        <td> Server{server_num + 1} </td>
                        <td> {format!("{:?}", values.0)} </td>
                        {move || if server_count() != 3 {Some(view! {<td> {format!("{:?}", values.1)} </td> } )} else {None}}
                    </tr>
                }).collect_view()
            }
        </table>
    }
}
