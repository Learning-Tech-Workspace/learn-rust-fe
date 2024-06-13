use yew::{function_component, html, Html, Properties};

use crate::components::message::Message;
use crate::types::Message as MessageType;

#[derive(PartialEq, Properties)]
pub struct BoxChatProps {
    pub messages: Vec<MessageType>,
}

#[function_component]
pub fn BoxChat(props: &BoxChatProps) -> Html {
    let BoxChatProps { messages } = props;
    html! {
        <div>
            {messages.iter().map(|msg| {
                html! {
                    <Message msg={msg.clone()} />
                }
            }).collect::<Html>()}
        </div>
    }
}
