use std::fmt::format;

use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub label: String,
    pub color: String,
}

#[function_component]
pub fn CategoryEntry(props: &Props) -> Html {
    let Props { id, label, color } = props.clone();

    html! {
      <div>
        <span>{ label }</span>
        <span style={format!("color: {}", color)}>{ color }</span>
      </div>
    }
}
