use tide::Request;

use crate::{
    state::State,
    todos::repository::repository::{
        assign_todo_to_category_db, create_todo_db, delete_todo_db, get_todo_db, get_todos_db,
        update_todo_db,
    },
};

use super::form::{CreateTodoDto, UpdateTodoCategoryDto, UpdateTodoDto};

async fn get_todos(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    // oops, I'm not returning the Dto, so the shape of the data is wrong
    let todos = get_todos_db(&pool).await?;

    Ok(serde_json::to_string_pretty(&todos)?)
}

async fn get_todo(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let id: i32 = req.param("id")?.parse()?;

    match get_todo_db(&pool, id).await {
        Ok(todo) => Ok(serde_json::to_string_pretty(&todo)?),
        Err(_) => Err(tide::Error::from_str(404, "Not found")),
    }
}

async fn create_todo(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let label = req.body_json::<CreateTodoDto>().await?.label;

    let todo = create_todo_db(&pool, &label).await?;
    Ok(serde_json::to_string_pretty(&todo)?)
}

async fn update_todo(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let id: i32 = req.param("id")?.parse()?;
    let completed = req.body_json::<UpdateTodoDto>().await?.completed;

    // FIXME I return Db in endpoins, instead of Dto
    let todo = update_todo_db(&pool, id, completed).await?;

    Ok(serde_json::to_string_pretty(&todo)?)
}

async fn assign_todo_to_category(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let id: i32 = req.param("id")?.parse()?;
    let category_id = req.body_json::<UpdateTodoCategoryDto>().await?.category_id;

    let todo = assign_todo_to_category_db(&pool, id, category_id).await?;

    Ok(serde_json::to_string_pretty(&todo)?)
}

async fn delete_todo(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let id: i32 = req.param("id")?.parse()?;

    let todo_db = delete_todo_db(&pool, id).await?;

    Ok(serde_json::to_string_pretty(&todo_db)?)
}

pub fn todo_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/").get(get_todos);
    api.at("/:id").get(get_todo);
    api.at("/").post(create_todo);
    // TODO "merge" the two patch endpoints
    api.at("/:id").patch(update_todo);
    api.at("/assign-to-category/:id")
        .patch(assign_todo_to_category);
    api.at("/:id").delete(delete_todo);

    api
}
