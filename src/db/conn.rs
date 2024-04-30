use dotenv::dotenv;
use std::env; // Import the dotenv crate

use rbatis::RBatis;
use rbdc_mysql::options::MySqlConnectOptions;
use rbdc_mysql::MysqlDriver;
use rbdc_pool_deadpool::DeadPool;

// fn get_conn_builder(
//     db_user: String,
//     db_password: String,
//     db_host: String,
//     db_port: u16,
//     db_name: String,
// ) -> mysql::OptsBuilder {
//     mysql::OptsBuilder::new()
//         .ip_or_hostname(Some(db_host))
//         .tcp_port(db_port)
//         .db_name(Some(db_name))
//         .user(Some(db_user))
//         .pass(Some(db_password))
// }

// pub fn create_pool() -> mysql::Pool {
//     dotenv().ok();

//     let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
//     let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
//     let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
//     let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
//     let db_name = env::var("MYSQL_DATABASE").expect("MYSQL_DBNAME is not set in .env file");
//     let db_port = db_port.parse().unwrap();

//     let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
//     mysql::Pool::new(builder).unwrap()
// }

pub async fn create_mysql_pool() -> RBatis {
    let mysql_pool = RBatis::new();

    dotenv().ok();

    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let db_name = env::var("MYSQL_DATABASE").expect("MYSQL_DBNAME is not set in .env file");
    let db_port = env::var("MYSQL_PORT")
        .expect("MYSQL_PORT is not set in .env file")
        .parse()
        .unwrap();

    let connection_option = MySqlConnectOptions::new()
        .host(&db_host)
        .username(&db_user)
        .password(&db_password)
        .database(&db_name)
        .port(db_port);

    mysql_pool
        .init_option::<MysqlDriver, MySqlConnectOptions, DeadPool>(
            MysqlDriver {},
            connection_option,
        )
        .unwrap();

    let pool = mysql_pool.get_pool().unwrap();
    pool.set_max_open_conns(15).await;

    mysql_pool
}
