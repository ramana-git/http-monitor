use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::error::Error;
use tiberius::time::chrono::NaiveDateTime;
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

pub async fn requests(pool: &Pool<ConnectionManager>, schema: &str) -> Result<Vec<HealthRequest>, Box<dyn Error>> {
    println!("Getting connection...from pool");
    let mut conn = pool.get().await.unwrap();
    println!("Got connection");
    let result: Vec<HealthRequest> = conn
        .simple_query(format!("select * from {schema}.HealthTrackers where active=1"))
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| -> HealthRequest {
            let headers = row.get::<&str, &str>("headers").unwrap();
            HealthRequest {
                uuid: row.get::<Uuid, &str>("tid").unwrap(),
                url: row.get::<&str, &str>("url").unwrap().to_owned(),
                headers: serde_json::from_str(headers).unwrap_or_default(),
                interval: row.get::<i32, &str>("interval").unwrap_or_default(),
                timeout: row.get::<i32, &str>("timeout").unwrap_or_default(),
                validation: match row.get::<&str, &str>("validation") {
                    Some("Json") => VType::Json,
                    Some("RegEx") => VType::RegEx,
                    Some("Text") => VType::Text,
                    _ => VType::None,
                },
                criteria: row.get::<&str, &str>("criteria").unwrap_or_default().to_owned(),
                condition: row.get::<&str, &str>("condition").unwrap_or_default().to_owned(),
            }
        })
        .collect();
    Ok(result)
}

pub async fn update_health(
    pool: &Pool<ConnectionManager>,
    schema: &str,
    time: i64,
    uuid: &Uuid,
    duration: i32,
    health: bool,
    code: i16,
    message: &String,
) {
    let time = NaiveDateTime::from_timestamp_millis(time).unwrap();
    let mut first8000 = message.as_str();
    if message.len() > 8000 {
        first8000 = &message[0..8000];
    }
    if let Ok(mut conn) = pool.get().await {
        let result = conn
            .execute(
                format!("insert into {schema}.HealthHistory values ((@P1), (@P2), (@P3), (@P4), (@P5), (@P6))"),
                &[uuid, &time, &duration, &health, &code, &first8000],
            )
            .await;
        println!("Updated HealthHistory: {:?}", result);
    } else {
        println!("Unable to connect to database");
    }
}
