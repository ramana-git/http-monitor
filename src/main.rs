use std::{fs, env};

use http_monitor::{default_headers, headers, request::HealthRequest, validate};
use reqwest::Client;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let file = if args.len() > 1 {
        args[1].as_str()
    } else {
        "src/requests.json"
    };
    println!("File: {file}");
    let contents = fs::read_to_string(file).unwrap();
    let requests: Vec<HealthRequest> = serde_json::from_str(&contents).unwrap();
    println!("{:#?}", requests);

    let mut handles = Vec::with_capacity(requests.len());
    for request in requests {
        let handle=tokio::spawn(async move {
            run_client(&request).await;
        });
        handles.push(handle);
    }
    println!("Handles: {:#?}", handles);
    for handle in handles {
        handle.await.unwrap();
    }
    Ok(())
}

async fn run_client(request: &HealthRequest) {
    let client = reqwest::Client::builder()
        .default_headers(default_headers())
        .timeout(Duration::from_secs(request.timeout))
        .build()
        .unwrap();
    let duration = Duration::from_secs(request.interval);
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
        validate(&response.text().await?, &request.validation);
    }
    Ok(())
}
