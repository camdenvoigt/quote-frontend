use leptos::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Quote {
    pub quote_id: i64,
    pub quote: String,
    pub author: String,
}

pub async fn fetch(endpoint: String) -> Result<Quote, Error> {
    use reqwasm::http::Request;

    let ep = format!(
        "http://localhost:3000/api/v1/{}",
        endpoint,
    );
    let result = Request::get(&ep)
        .send()
        .await?
        // convert it to JSON
        .json()
        .await?;
    Ok(result)
}
