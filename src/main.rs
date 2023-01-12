use std::{env, error::Error, time::{SystemTime, UNIX_EPOCH}};

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use http_monitor::{
    default_headers, headers,
    request::HealthRequest,
    trackers::{connect, requests, update_health},
    validate,
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = match env::var("SERVER") {
        Ok(value) => value,
        Err(_e) => "localhost".to_owned(),
    };
    let port = match env::var("PORT") {
        Ok(value) => value,
        Err(_e) => "1433".to_owned(),
    };
    let database = match env::var("DATABASE") {
        Ok(value) => value,
        Err(_e) => "master".to_owned(),
    };
    let user = match env::var("DB_USER") {
        Ok(value) => value,
        Err(_e) => "sa".to_owned(),
    };
    let password = match env::var("DB_PASSWORD") {
        Ok(value) => value,
        Err(_e) => "YourStrong!Passw0rd".to_owned(),
    };
    let max_pool_size = match env::var("DB_MAX_POOL_SIZE") {
        Ok(value) => value.parse::<u8>().unwrap(),
        Err(_e) => 3,
    };
    let conn_str = format!(
        "Server={server};Port={port};Database={database};User Id={user};Password={password};"
    );
    println!("Connection string: {}", conn_str);

    let pool = connect(&conn_str, max_pool_size).await.unwrap();
    monitor(&pool).await;
    Ok(())
}

async fn monitor(pool: &Pool<ConnectionManager>) {
    let trackers = requests(&pool).await.unwrap();
    println!("{:#?}", trackers);
    let mut handles = Vec::with_capacity(trackers.len());
    for request in trackers {
        let pool = pool.clone();
        let handle = tokio::spawn(async move {
            run_client(&pool, &request).await;
        });
        handles.push(handle);
    }
    println!("Handles: {:#?}", handles);
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn run_client(pool: &Pool<ConnectionManager>, request: &HealthRequest) {
    let client = reqwest::Client::builder()
        .default_headers(default_headers())
        .timeout(Duration::from_secs(request.timeout.try_into().unwrap()))
        .build()
        .unwrap();
    let duration = Duration::from_secs(request.interval.try_into().unwrap());

    for i in 1.. {
        let builder = client.get(&request.url).headers(headers(&request.headers));
        let response = builder.send().await.unwrap();
        let status=response.status();
        let headers = response.headers().clone();
        let body = response.text().await.unwrap(); 
        let mut health=false;
        if status.is_success() {
            health=validate(
                &body,
                &request.validation,
                &request.criteria,
                &request.condition,
            );
        }
        let code=status.as_u16();
        let time=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let message=format!("[{:#?},{body}]",headers);
        println!("time# {i} - {health} - {status} - {:#?} - {:#?}", headers,body);
        update_health(&pool, time as i64, &request.uuid, health, code as i16, &message).await;
        sleep(duration).await;
    }
}