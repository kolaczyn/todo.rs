mod components;

use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

use crate::components::{category_entry::CategoryEntry, todo_entry::TodoEntry};

// TODO this is copied from the backend source code, but ideally, frontend and backend should share the same type
#[derive(Deserialize)]
pub struct TodoDto {
    pub id: i32,
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct CategoriesDto {
    pub id: i32,
    pub label: String,
    pub color: String,
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
                    let fetched_videos: Vec<TodoDto> =
                        Request::get("http://localhost:8080/v1/todos")
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

    let categories = use_state(|| vec![]);
    {
        let categories = categories.clone();
        use_effect_with_deps(
            move |_| {
                let categories = categories.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_categories: Vec<CategoriesDto> =
                        Request::get("http://localhost:8080/v1/categories")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    categories.set(fetched_categories);
                });
                || ()
            },
            (),
        );
    }

    let categories_list = categories.iter().map(|x| {
        html! {
        <CategoryEntry id={x.id} label={x.label.to_owned()} color={x.color.to_owned()} />
            }
    });

    html! {
        <div>
            <span>{ "You have" }</span>
            <span>{ videos.len() }</span>
            <span>{ "todos" }</span>
            {for videos_list}
            {for categories_list}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
