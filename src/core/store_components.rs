use std::collections::HashMap;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TestProps {}

#[function_component]
pub fn Test(props: &TestProps) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

pub fn store_yew_components() -> HashMap<String, Html> {
    let mut component_collection = HashMap::new();
    let test_html = html! {
        <Test />
    };
    component_collection.insert("Test".to_string(), test_html);
    component_collection
}
