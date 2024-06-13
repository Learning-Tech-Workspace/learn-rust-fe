use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub content: Option<String>,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let TitleProps { content } = props;
    html! {
        <>
            if let Some(content) = content {
                <h1>{content}</h1>
            } else {
                <h1>{"Front End App"}</h1>
            }
        </>
    }
}
