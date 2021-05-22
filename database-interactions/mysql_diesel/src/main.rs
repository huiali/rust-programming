use diesel::RunQueryDsl;
use diesel::MysqlConnection;
use diesel::{sql_query, Connection};

fn main() {
    let connection = establish_connection();
    // connection::sql_query();
    // let rows: _ = sql_query(r#"SELECT * FROM todos"#).load::<_>(&connection);;

    println!("{:?}", rows);
}

pub fn establish_connection() -> MysqlConnection {
    let database_url: &str = "mysql://root:huiali123@192.168.164.131:3306/todos";
    MysqlConnection::establish(database_url).expect("msg: &str")
}
