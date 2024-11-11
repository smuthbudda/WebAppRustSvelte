use sqlx::migrate::MigrateError;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
use std::process::exit;

/// Creates a connection pool for the PSQL database
pub async fn connect_to_database() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database url: {}", database_url);
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            exit(1);
        }
    };

    // Attempt to run migrations
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!("✅ Database migrations applied successfully"),
        Err(MigrateError::VersionMismatch { .. }) => {
            println!("❌ Migration failed due to version mismatch");

            // Attempt to drop and recreate the database
            if drop_and_recreate_database(&database_url).await.is_ok() {
                println!("🔄 Database dropped and recreated, retrying migrations...");
                if let Err(err) = sqlx::migrate!().run(&pool).await {
                    println!(
                        "❌ Failed to apply database migrations after recreation: {:?}",
                        err
                    );
                    exit(1);
                } else {
                    println!("✅ Database migrations applied successfully after recreation");
                }
            } else {
                println!("❌ Failed to drop and recreate the database");
                exit(1);
            }
        }
        Err(err) => {
            println!("❌ Failed to apply database migrations: {:?}", err);
            exit(1);
        }
    }

    pool
}

async fn drop_and_recreate_database(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Extract the database name from the URL
    let (base_url, db_name) = parse_database_url(database_url)?;

    // Connect to the PostgreSQL server (without specifying the database)
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&base_url)
        .await?;
    pool.execute(format!("REVOKE CONNECT ON DATABASE {} FROM PUBLIC;", db_name).as_str())
        .await?;
    pool.execute(
        format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            db_name
        )
        .as_str(),
    )
    .await?;
    // Drop the existing database
    pool.execute(format!("DROP DATABASE IF EXISTS {}", db_name).as_str())
        .await?;

    // Recreate the database
    pool.execute(format!("CREATE DATABASE {}", db_name).as_str())
        .await?;

    Ok(())
}

fn parse_database_url(database_url: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let url_parts: Vec<&str> = database_url.rsplitn(2, '/').collect();
    if url_parts.len() != 2 {
        return Err("Invalid DATABASE_URL format".into());
    }
    let base_url = url_parts[1];
    let db_name = url_parts[0];
    Ok((base_url.to_string(), db_name.to_string()))
}
