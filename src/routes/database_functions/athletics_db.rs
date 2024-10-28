use std::error::Error;

use std::time::SystemTime;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use sqlx::{Pool, Postgres};
use crate::models::iaaf_points::PointsInsert;

pub async fn read_into_db(pool : &Pool<Postgres>) -> bool{
    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(id) FROM points"#)
        .fetch_one(pool)
        .await
        .unwrap();

    if count > 200000 {
        return false;
    }

    let models: Result<Vec<PointsInsert>, Box<dyn Error>> = read_file_async().await;

    return match models {
        Err(e) => {
            false
        }
        Ok(_) => {
            print!("inserting into database");
            //There has to be a better way to insert 200000 records than individually. It takes so long.
            let models: Vec<PointsInsert> = models.unwrap_or_default();

            let points: Vec<i32> = models.iter().map(|p| p.points).collect();
            let genders: Vec<String> = models.iter().map(|p| p.gender.clone()).collect();
            let categories: Vec<String> = models.iter().map(|p| p.category.clone()).collect();
            let events: Vec<String> = models.iter().map(|p| p.event.clone()).collect();
            let marks: Vec<f64> = models.iter().map(|p| p.mark).collect();
            let start = SystemTime::now();
            let query_result = sqlx::query(
                r#"INSERT INTO points (points, gender, category, event, mark)
                    SELECT * FROM UNNEST($1::INTEGER[], $2::VARCHAR(10)[], $3::VARCHAR(20)[], $4::VARCHAR(10)[], $5::Float[])"#,
            )
                .bind(points)
                .bind(genders)
                .bind(categories)
                .bind(events)
                .bind(marks)
                .execute(pool)
                .await;

            match query_result {
                Ok(_) => true,
                Err(_) => false
            }
        }
    };

    static FILE_LOCATION: &str = "data/WorldAthletics.json";

    async fn read_file_async() -> Result<Vec<PointsInsert>, Box<dyn Error>> {
        let file = File::open(FILE_LOCATION).await?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;

        let points: Vec<PointsInsert> = serde_json::from_slice(&buffer)?;

        Ok(points)
    }

}