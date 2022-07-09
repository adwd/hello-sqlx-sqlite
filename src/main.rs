use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:example.db3")
        .await?;

    let db = DB { pool };

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db.pool)
        .await?;

    assert_eq!(row.0, 150);

    let row2 = sqlx::query!("select * from person")
        .fetch_all(&db.pool)
        .await?;

    let person = &row2[0];
    println!("{:?}", person);

    let person2 = db.get_person(3).await?;
    println!("{:?}", person2);

    let sales_people = db.get_sales().await?;
    println!("{:?}", sales_people);

    Ok(())
}

#[derive(Debug)]
struct Person {
    id: i64,
    name: String,
    department: String,
    salary: i64,
}

struct DB {
    pool: Pool<Sqlite>,
}

impl DB {
    async fn get_person(&self, id: i32) -> Result<Person, sqlx::Error> {
        let person = sqlx::query_as!(Person, "select * from person where id = ?1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(person)
    }

    async fn get_sales(&self) -> Result<Vec<Person>, sqlx::Error> {
        let people = sqlx::query_as!(Person, "select * from person where department = 'sales'")
            .fetch_all(&self.pool)
            .await?;

        Ok(people)
    }
}
