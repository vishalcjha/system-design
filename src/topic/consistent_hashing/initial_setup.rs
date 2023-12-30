use std::collections::BTreeMap;

use leptos::*;
fn get_initail() -> BTreeMap<u32, (Vec<u32>, Vec<u32>)> {
    (1..12).fold(
        BTreeMap::<u32, (Vec<u32>, Vec<u32>)>::new(),
        |mut accum, current| {
            accum
                .entry(current % 3)
                .and_modify(|nums| nums.0.push(current))
                .or_insert_with(|| (vec![current], Vec::new()));
            accum
        },
    )
}

#[component]
pub(super) fn InitialSetupComponent(server_count: ReadSignal<u32>) -> impl IntoView {
    let setup = move || {
        if server_count() != 3 {
            let mut initial = get_initail();
            for i in 1..12 {
                initial
                    .entry(i % server_count())
                    .and_modify(|nums| nums.1.push(i))
                    .or_insert_with(|| (Vec::new(), vec![i]));
            }
            initial
        } else {
            get_initail()
        }
    };

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
