use yew::prelude::*;

use crate::core::store_components::store_yew_components;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            { for store_yew_components().iter().map(|(key, component)| {
                html! {
                    <div>
                        {<yew::virtual_dom::VNode as Clone>::clone(component)}
                    </div>
                }
            })}
        </div>
    }
}
