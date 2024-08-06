use anyhow::Context as _;
use sqlx::mysql::MySqlConnection;
use sqlx::Connection as _;
use std::env;
use uuid::{fmt::Hyphenated, Uuid};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").context("missing `DATABASE_URL`")?;
    let mut conn = MySqlConnection::connect(&database_url)
        .await
        .context("connect to database")?;

    sqlx::migrate!()
        .run(&mut conn)
        .await
        .context("run migrations")?;

    match sqlx::query_as::<_, FooUuid>("SELECT bar FROM Foo")
        .fetch_one(&mut conn)
        .await
    {
        Ok(foo) => println!("Fetched `FooUuid`: {foo:?}"),
        Err(err) => eprintln!("Failed to fetch `FooUuid`: {err}"),
    }

    match sqlx::query_as::<_, FooHyphenated>("SELECT bar FROM Foo")
        .fetch_one(&mut conn)
        .await
    {
        Ok(foo) => println!("Fetched `FooHyphenated`: {foo:?}"),
        Err(err) => eprintln!("Failed to fetch `FooHyphenated`: {err}"),
    }

    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct FooUuid {
    #[allow(dead_code)]
    bar: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
struct FooHyphenated {
    #[allow(dead_code)]
    bar: Hyphenated,
}
