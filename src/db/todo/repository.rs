use rbatis::rbdc::db::ExecResult;
use rbatis::RBatis;

use super::schema::Todo;

pub struct TodoRepository {
    mysql_pool: RBatis,
}

impl TodoRepository {
    pub fn new(mysql_pool: RBatis) -> Self {
        Self { mysql_pool }
    }

    pub async fn insert_into_todo(&self, data: Todo) -> Result<ExecResult, rbatis::rbdc::Error> {
        Todo::insert(&self.mysql_pool, &data).await
    }

    pub async fn select_todo(&self, id: i32) -> Result<Option<Todo>, rbatis::rbdc::Error> {
        Todo::select_by_id(&self.mysql_pool, id).await
    }

    pub async fn select_all(&self) -> Result<Vec<Todo>, rbatis::rbdc::Error> {
        Todo::select_all(&self.mysql_pool).await
    }

    pub async fn put_todo(&self, data: Todo) -> Result<ExecResult, rbatis::rbdc::Error> {
        Todo::update_by_column(&self.mysql_pool, &data, "id").await
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> Result<ExecResult, rbatis::rbdc::Error> {
        Todo::delete_by_column(&self.mysql_pool, "id", &id.to_string()).await
    }
}
