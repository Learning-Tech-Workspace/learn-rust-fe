use std::ops::Deref;

use yew::prelude::*;

use crate::components::box_chat::BoxChat;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::title::Title;
use crate::types::Message;

#[function_component(App)]
pub fn app() -> Html {
    let message = use_state(|| "".to_string());

    let messages = vec![
        Message {
            content: "Hello Ben!".to_string(),
            author: "Alex".to_string(),
        },
        Message {
            content: "Hi Alex!".to_string(),
            author: "Ben".to_string(),
        },
    ];
    html! {
          <>
                <Title content={Some("FE App")}/>
                <BoxChat messages={messages} />
                <Input r#type="text" state={message.clone()} />
                <Button label="Send"/>
                <p>{message.deref().to_string()}</p>
          </>
    }
}
