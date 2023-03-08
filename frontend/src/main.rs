mod components;

use components::TodoEntry;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

// TODO this is copied from the backend source code, but ideally, frontend and backend should share the same type
#[derive(Deserialize)]
pub struct Todo {
    pub id: i64,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[function_component]
fn App() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Todo> = Request::get("http://localhost:8080/todos")
                        .send()
                        .await
                        // TODO remove unwraps and make videos Result
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let videos_list = videos.iter().map(|x| {
        html! {
            // will have to get rid of those to_owned()
        <TodoEntry id={x.id} label={x.label.to_owned()} description={x.description.to_owned()} completed={x.completed} />
            }
    });

    html! {
        <div>
            <span>{ "You have" }</span>
            <span>{ videos.len() }</span>
            <span>{ "todos" }</span>
            {for videos_list}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
