use chrono::Local;
use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::mpsc;
use std::{thread, time};

fn get_public_ip() -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()
        .unwrap();
    println!("{:#?}", resp);
    let ip = resp.get("origin").map(|s| s.to_string()).unwrap();
    Ok(ip)
}

fn delete_record(body: &Vec<Record>, client: &Client, zone: &String) {
    let find_record = &body
        .iter()
        .find(|r| r.hostname == env::var("DOMAIN").expect("DOMAIN must be an env variable"));
    let id = match find_record {
        Some(r) => r.id.as_ref().unwrap(), // as_ref converts from &Option<Record> to Option<&Record>
        None => {
            println!("Record not found, nothing to delete");
            return;
        }
    };
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be an env variable");
    let delete_api = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records/{}?access_token={}",
        zone, id, access_token
    );
    let resp = client.delete(delete_api).send().unwrap();
    println!("Delete resp status:{}", resp.status());
}

fn post_record(val: &str, client: &Client, zone: &String) {
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be an env variable");
    let body = Record {
        hostname: env::var("DOMAIN")
            .expect("DOMAIN must be an env variable")
            .into(),
        type_: "A".to_string(),
        ttl: 3600,
        value: val.to_string(),
        id: None,
    };
    let post_api = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records?access_token={}",
        zone, access_token
    );
    let resp = client.post(post_api).json(&body).send().unwrap();
    println!("Post resp status:{}", resp.status());
}

fn get_records(client: &Client, zone: &String) -> Vec<Record> {
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be an env variable");
    let dns_api = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records?access_token={}",
        zone, access_token
    );
    let resp = client.get(&dns_api).send().unwrap();
    if resp.status() == StatusCode::OK {
        let body = resp.json::<Vec<Record>>().unwrap();
        println!("Body {:?}", body);
        body
    } else {
        panic!("call: {} \nStatus code: {}", &dns_api, resp.status());
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    hostname: String,
    #[serde(rename = "type")]
    type_: String,
    id: Option<String>,
    ttl: u32,
    value: String,
}

fn check_dns() {
    let public_ip = match get_public_ip() {
        Ok(ip) => ip,
        Err(error) => {
            println!("Error: {}. Did not get the public ip, returning.", error);
            return;
        }
    };
    dotenv().ok();
    let zone = env::var("ZONE").expect("ZONE must be an env variable");
    let client = Client::new();
    let body = get_records(&client, &zone);
    let maybe_record = &body
        .iter()
        .find(|&r| r.hostname == env::var("DOMAIN").expect("DOMAIN must be an env variable"));
    let ip = match maybe_record {
        // maybe the record is missing
        Some(r) => &r.value,
        None => "Not found",
    };
    println!("dns ip: {:?}", ip);
    if ip != public_ip {
        println!("ip changed!");
        delete_record(&body, &client, &zone);
        post_record(&public_ip, &client, &zone);
        get_records(&client, &zone);
    }
}

fn main() {
    // total overkill, but I love it.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        tx.send(()).unwrap();
        thread::sleep(time::Duration::from_secs(300));
    });
    loop {
        rx.recv().unwrap();
        println!("Checking dns. Time: {}", Local::now());
        check_dns();
    }
}
