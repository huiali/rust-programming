use sqlx::MySqlPool;
use sqlx::{mysql::MySqlPoolOptions, query};
// use std::env;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "mysql://root:huiali123@192.168.164.131:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new().connect(database_url).await?;

    let rows = query!("select `id`,`key`,`description` from todos")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", rows);
    for rec in rows {
        println!(
            "- {}: {},[{:?}]",
            //if rec.done != 0 { "x" } else { " " },
            rec.id,
            &rec.key,
            &rec.description,
        );
    }
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

async fn list_todos(pool: &MySqlPool) -> anyhow::Result<()> {
    let recs = sqlx::query!("SELECT id, description FROM todos ORDER BY id")
        .fetch_all(pool)
        .await?;

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
    //       0 = false, non-0 = true
    for rec in recs {
        println!("- [{}] {:?}", rec.id, &rec.description,);
    }

    Ok(())
}
