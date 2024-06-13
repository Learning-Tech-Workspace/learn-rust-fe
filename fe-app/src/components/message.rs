use yew::{function_component, html, Html, Properties};

use crate::types::Message as MessageType;

#[derive(PartialEq, Properties)]
pub struct MessageProps {
    pub msg: MessageType,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    let MessageProps { msg } = props;
    let MessageType { content, author } = msg;
    html! {
        <div>
            <h3>{author}</h3>
            <p>{content}</p>
        </div>
    }
}
