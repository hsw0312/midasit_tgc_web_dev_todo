use rbatis::crud;
use serde::{Deserialize, Serialize};

use rbatis::impl_select;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub content: Option<String>,
    pub done: Option<bool>,
}

crud!(Todo {});
impl_select!(Todo{select_by_id(id:i32) -> Option => "`where id = #{id} limit 1`"});
