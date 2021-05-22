use mysql::{prelude::Queryable, Pool};

fn main() {
    let database_url = "mysql://root:huiali123@192.168.164.131:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = Pool::new(database_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let sql = "select `id`,`key`,`description` AS description from todos";
    let val = &conn.query_iter(sql).unwrap();
    println!("{:?}", val);
}
