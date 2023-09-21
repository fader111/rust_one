use reqwest::{Error, Response};
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

fn main() -> Result<(), reqwest::blocking::BlockingError> {

    // Создание URL
    let url = "https://www.rust-lang.org/".parse::<SocketAddr>()?;

    println!("Содержимое страницы по адресу {}", url);

    let client = reqwest::Client::new();
    let resp = client.get(url).send().map_err(|e| io::Error::new(e.classify(), e))?;
    if resp.status() != StatusCode::OK {
        return Err(io::Error::from(resp.status()));
    }

    print!("{}", resp.text().unwrap_or_else(|_| "Ошибка получения содержимого страницы".to_string()));

    Ok(())
}