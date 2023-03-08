use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[function_component]
pub fn TodoEntry(props: &Props) -> Html {
    html! {
      <div>
        <span>{ props.id }</span>
        <span>{ props.label.to_owned() }</span>
      </div>
    }
}
