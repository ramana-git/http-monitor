use std::{env, error::Error, fs};

use http_monitor::{
    default_headers, headers,
    request::HealthRequest,
    trackers::{connect, requests},
    validate,
};
use reqwest::Client;
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

    let conn_str = format!(
        "Server={server};Port={port};Database={database};User Id={user};Password={password};"
    );
    println!("Connection string: {}", conn_str);
    read_from_db(&conn_str).await
}

async fn read_from_db(conn_str: &str) -> Result<(), Box<dyn Error>> {
    let max_pool_size = match env::var("DB_MAX_POOL_SIZE") {
        Ok(value) => value.parse::<u8>().unwrap(),
        Err(_e) => 3,
    };
    let pool = connect(conn_str, max_pool_size).await?;
    let trackers = requests(&pool).await?;
    println!("{:#?}", trackers);
    monitor(trackers).await;
    Ok(())
}

async fn _read_from_file() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = match args.len() {
        1 => "src/requests.json",
        _ => args[1].as_str(),
    };
    println!("File: {file}");
    let contents = fs::read_to_string(file).unwrap();
    let trackers: Vec<HealthRequest> = serde_json::from_str(&contents).unwrap();
    println!("{:#?}", trackers);

    monitor(trackers).await;
    Ok(())
}

async fn monitor(trackers: Vec<HealthRequest>) {
    let mut handles = Vec::with_capacity(trackers.len());
    for request in trackers {
        let handle = tokio::spawn(async move {
            run_client(&request).await;
        });
        handles.push(handle);
    }
    println!("Handles: {:#?}", handles);
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn run_client(request: &HealthRequest) {
    let client = reqwest::Client::builder()
        .default_headers(default_headers())
        .timeout(Duration::from_secs(request.timeout.try_into().unwrap()))
        .build()
        .unwrap();
    let duration = Duration::from_secs(request.interval.try_into().unwrap());
    for i in 1.. {
        let status = check_status(&client, &request).await;
        println!("time# {i} - {:#?}", status);
        sleep(duration).await;
    }
}

async fn check_status(client: &Client, request: &HealthRequest) -> Result<(), reqwest::Error> {
    let builder = client.get(&request.url).headers(headers(&request.headers));
    let response = builder.send().await?;
    println!("{:#?}", response);
    let status = response.status().is_success();
    println!("{status}");
    if status {
        validate(
            &response.text().await?,
            &request.validation,
            &request.criteria,
            &request.condition,
        );
    }
    Ok(())
}
