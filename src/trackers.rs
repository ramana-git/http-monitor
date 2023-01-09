use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::error::Error;
use uuid::Uuid;

use crate::request::{HealthRequest, VType};

pub async fn connect(conn_str: &str, max_size: u8) -> Result<Pool<ConnectionManager>, Box<dyn Error>> {
    let mgr = ConnectionManager::build(conn_str)?;
    let pool = Pool::builder().max_size(max_size.into()).build(mgr).await?;
    Ok(pool)
}

pub async fn requests(pool: &Pool<ConnectionManager>) -> Result<Vec<HealthRequest>, Box<dyn Error>> {
    let mut conn = pool.get().await?;
    let result: Vec<HealthRequest> = conn
        .simple_query("select * from HealthTrackers where active=1")
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| -> HealthRequest {
            HealthRequest {
                uuid: row.get::<Uuid, usize>(0).unwrap(),
                url: row.get::<&str, usize>(1).unwrap().to_string(),
                headers: serde_json::from_str(row.get::<&str, usize>(1).unwrap()).unwrap(),
                interval: row.get::<i64, usize>(3).unwrap(),
                timeout: row.get::<i64, usize>(4).unwrap(),
                validation: match row.get::<&str, usize>(5) {
                    Some("Json") => VType::Json,
                    Some("RegEx") => VType::RegEx,
                    Some("Text") => VType::Text,
                    _ => VType::Text,
                },
                criteria: row.get::<&str, usize>(6).unwrap().to_string(),
                condition: row.get::<&str, usize>(7).unwrap().to_string(),
            }
        })
        .collect();
    Ok(result)
}

pub async fn update_health(pool: &Pool<ConnectionManager>, time: i64, uuid: &Uuid, status: bool, code: i16, message: &String) {
    let first8000= &message[0..8000];
    if let Ok(mut conn) = pool.get().await {
        let result = conn
            .execute(
                "insert into HealthHistory values (@P1), (@P2), (@P3), (@P4), (@P5)",
                &[uuid, &time, &status, &code, &first8000],
            ).await;
        println!("Updated HealthHistory: {:?}", result);
    } else {
        println!("Unable to connect to database");
    }
}