use rbatis::crud;
use serde::{Deserialize, Serialize};

use rbatis::impl_select;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub content: Option<String>,
    pub done: Option<i8>,
}

crud!(Todo {});
impl_select!(Todo{select_by_id(id:i32) -> Option => "`where id = #{id} limit 1`"});

use rbatis::table_sync::MysqlTableMapper;
use rbatis::RBatis;
use rbs::to_value;

impl Todo {
    pub async fn sync_table_todo(mysql_pool: &RBatis) {
        _ = RBatis::sync(
            &mysql_pool.acquire().await.unwrap(),
            &MysqlTableMapper {},
            /*&Todo{
                id : (0),
                content : Some(String::new()),
                done : Some(false),
            },*/
            &to_value! {
                "id":"INTEGER PRIMARY KEY AUTO_INCREMENT",
                "content":"varchar(255) NOT NULL",
                "done":"bool NOT NULL",
            },
            "todo",
        )
        .await
    }
}
