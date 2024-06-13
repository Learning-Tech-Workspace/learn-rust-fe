use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub r#type: String,
    pub state: UseStateHandle<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let state = props.state.clone();

    let on_change = Callback::from(move |evt: Event| {
        let target = evt.target().unwrap(); // We can safely unwrap here because we know that the target is an input element
        let input = target.unchecked_into::<HtmlInputElement>();
        state.set(input.value());
    });

    let r#type = props.r#type.deref().to_string();

    html! {
        <input type={r#type} onchange={on_change} />
    }
}
