use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::time::chrono::NaiveDateTime;
use std::error::Error;
use uuid::Uuid;

use crate::request::{HealthRequest, VType};

pub async fn connect(
    conn_str: &str,
    max_size: u8,
) -> Result<Pool<ConnectionManager>, Box<dyn Error>> {
    let mgr = ConnectionManager::build(conn_str)?;
    let pool = Pool::builder().max_size(max_size.into()).build(mgr).await?;
    Ok(pool)
}

pub async fn requests(pool: &Pool<ConnectionManager>) -> Result<Vec<HealthRequest>, Box<dyn Error>> {
    println!("Getting connection...from pool");
    let mut conn = pool.get().await.unwrap();
    println!("Got connection");
    let result: Vec<HealthRequest> = conn
        .simple_query("select * from HealthTrackers where active=1")
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| -> HealthRequest {
            let headers = row.get::<&str, usize>(2).unwrap();
            HealthRequest {
                uuid: row.get::<Uuid, usize>(0).unwrap(),
                url: row.get::<&str, usize>(1).unwrap().to_owned(),
                headers: serde_json::from_str(headers).unwrap_or_default(),
                interval: row.get::<i32, usize>(3).unwrap_or_default(),
                timeout: row.get::<i32, usize>(4).unwrap_or_default(),
                validation: match row.get::<&str, usize>(5) {
                    Some("Json") => VType::Json,
                    Some("RegEx") => VType::RegEx,
                    Some("Text") => VType::Text,
                    _ => VType::None,
                },
                criteria: row.get::<&str, usize>(6).unwrap_or_default().to_owned(),
                condition: row.get::<&str, usize>(7).unwrap_or_default().to_owned(),
            }
        })
        .collect();
    Ok(result)
}

pub async fn update_health(
    pool: &Pool<ConnectionManager>,
    time: i64,
    uuid: &Uuid,
    health: bool,
    code: i16,
    message: &String,
) {
    let time=NaiveDateTime::from_timestamp_millis(time).unwrap();
    let mut first8000=message.as_str();
    if message.len()>8000{
        first8000=&message[0..8000];
    }
    if let Ok(mut conn) = pool.get().await {
        let result = conn
            .execute(
                "insert into HealthHistory values ((@P1), (@P2), (@P3), (@P4), (@P5))",
                &[uuid, &time, &health, &code, &first8000],
            )
            .await;
        println!("Updated HealthHistory: {:?}", result);
    } else {
        println!("Unable to connect to database");
    }
}
