use mysql::prelude::*;
use mysql::*;

fn main() {
    let database_url = "mysql://root:huiali123@192.168.164.131:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = Pool::new(database_url).unwrap();
    let conn = &mut pool.get_conn().unwrap();
    let sql = "select `id`,`key`,`description` AS description from todos";
    let val = &mut conn.query_iter(sql).unwrap().for_each(|f| {
        let row1: _ = from_row(f.unwrap());
        println!("{:?}",);
    });

    // for item in val {
    //     if let Ok(i) = item {
    //         println!("{:?}", from_row(i.unwrap()));
    //     };
    // }
}
