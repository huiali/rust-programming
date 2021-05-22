use mysql_async::Error;
use mysql_async::{prelude::*};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let database_url: &str = "mysql://root:huiali123@192.168.164.131:3306/todos";
    let conn = mysql_async::Pool::new(database_url).get_conn().await?;
    let result: _ = conn.query("SELECT * FROM todos").await?;
    println!("{:?}", result);
    Ok(())
}
