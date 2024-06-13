use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub label: String,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { label } = props;
    html! {
        <button>{label}</button>
    }
}
