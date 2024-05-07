use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub content: String,
}
