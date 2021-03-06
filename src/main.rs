use std::{
    env,
    result::Result,
};
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use serde_json::json;
use reqwest::{StatusCode};
use std::collections::HashMap;

fn array_from(comma_separated_string: String) -> Vec<String> {
    comma_separated_string.split(",")
        .map(|it| it.trim().into() )
        .collect()
}

fn map_from(comma_separated_string: String) -> HashMap<String, String> {
    comma_separated_string.split(',')
        .map(|kv| kv.split('='))
        .map(|mut kv| (kv.next().unwrap().trim().into(),
                       kv.next().unwrap().trim().into()))
        .collect()
}


#[tokio::main]
pub async fn main() -> Result<(), String> {
    let client = reqwest::Client::new();
    let consul_url = env::var("CONSUL_URL").unwrap_or("http://localhost:8500".to_string());
    //https://www.consul.io/api-docs/agent/service
    let json = json!({
        "Name": env::var("REG_SERVICE_NAME").expect("Service name is required argument!"),
        "ID": env::var("REG_INSTANCE_NAME").expect("Service instance name is required!"),
        "Address": env::var("REG_INSTANCE_ADDR").expect("Service instance address is required!"),
        "Port": env::var("REG_INSTANCE_PORT")
                    .expect("Service instance port is required!")
                    .parse::<usize>()
                    .expect("Port should be a number"),
        "Tags": array_from(env::var("REG_INSTANCE_TAGS").unwrap_or("".into())),
        "Meta": map_from(env::var("REG_INSTANCE_META").unwrap_or("".into())),
        "Check": {
            "DeregisterCriticalServiceAfter": env::var("REG_HEALTHCHECK_DEREGISTER_AFTER").unwrap_or("".into()),
            "HTTP": env::var("REG_HEALTHCHECK_URL").unwrap_or("".into()),
            "Interval": env::var("REG_HEALTHCHECK_INTERVAL").unwrap_or("1m".into()),
            "Timeout": env::var("REG_HEALTHCHECK_TIMEOUT").unwrap_or("5s".into())
        }
    });
    let res = client.put(&format!("{}/v1/agent/service/register", consul_url))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .json(&json)
        .send().await
        .expect("Unable to register the service");
    if res.status()==StatusCode::OK { Ok(())} else {Err(format!("Consul agent returned unexpected code: {}", res.status()))}
}