use sqlx::{mysql::MySqlRow, FromRow};

use sqlx::Row;
use sqlx::{mysql::MySqlPoolOptions, query};
// use std::env;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "mysql://root:huiali123@192.168.164.131:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new().connect(database_url).await?;

    let sql = "select `id`,`key`,`description` AS description from todos";
    let rows = query(sql).map(|row: MySqlRow| row).fetch_all(&pool).await?;
    let rows = sqlx::query_unchecked!(
        "select `id`,`key`,`description` AS description from todos where id = ?",
        1
    )
    .fetch_all(&pool) // -> Vec<{ country: String, count: i64 }>
    .await?;
    println!("{:?}", rows);


    // let row: _ = sqlx::query_as::<_, O>("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool).await?;

    // let columns: Vec<&str> = Vec::new();
    // // println!("{:?}", rows);
    // for row in rows {
    //     // let id: i32 = row.get(0);
    //     println!("{:?}", &row.column(index));
    //     println!("-----------------------------");
    // }
    //let description = String::from("the todo description");
    //let todo_id = add_todo(&pool, description).await?;
    //list_todos(&pool).await?;
    // complete_todo(&pool, id).await?

    Ok(())
}

// async fn add_todo(pool: &MySqlPool, description: String) -> anyhow::Result<u64> {
//     // Insert the task, then obtain the ID of this row
//     let todo_id = sqlx::query!(
//         r#"
// INSERT INTO todos ( description )
// VALUES ( ? )
//         "#,
//         description
//     )
//     .execute(pool)
//     .await?
//     .last_insert_id();

//     Ok(todo_id)
// }

// async fn complete_todo(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
//     let rows_affected = sqlx::query!(
//         r#"
// UPDATE todos
// SET done = TRUE
// WHERE id = ?
//         "#,
//         id
//     )
//     .execute(pool)
//     .await?
//     .rows_affected();

//     Ok(rows_affected > 0)
// }

// async fn list_todos(pool: &MySqlPool) -> anyhow::Result<()> {
//     let recs = sqlx::query!("SELECT id, description FROM todos ORDER BY id")
//         .fetch_all(pool)
//         .await?;

//     // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
//     //       0 = false, non-0 = true
//     for rec in recs {
//         println!("- [{}] {:?}", rec.id, &rec.description,);
//     }

//     Ok(())
// }
