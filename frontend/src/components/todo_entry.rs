use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[function_component]
pub fn TodoEntry(props: &Props) -> Html {
    let Props {
        completed,
        description,
        id,
        label,
    } = props.clone();

    let completed_message = if *completed { "[x]" } else { "[ ]" };

    html! {
      <div>
        <span>{ id }</span>
        <span>{ completed_message }</span>
        <span>{ label.to_owned() }</span>
      </div>
    }
}
