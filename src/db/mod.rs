pub mod conn;
pub mod todo;
use rbatis::RBatis;

#[derive(Clone)]
pub struct AppState {
    pub mysql_pool: RBatis,
}

pub async fn sync_table(mysql_pool: &RBatis) {
    todo::schema::Todo::sync_table_todo(mysql_pool).await;
}
