pub mod conn;
pub mod todo;

use rbatis::RBatis;

#[derive(Clone)]
pub struct AppState {
    pub mysql_pool: RBatis,
}
