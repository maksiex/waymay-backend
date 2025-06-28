
use axum::{Json, http::StatusCode};
use reqwest::Client;
use std::env;
use crate::models::place::Place;
pub async fn get_city_info() -> Result<Json<Vec<Place>>, StatusCode> {
    let api_key = env::var("API_NINJA_KEY")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let client = Client::new();
    
    let response = client
        .get("https://api.api-ninjas.com/v1/city?name=Moscow")
        .header("X-Api-Key", api_key)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !response.status().is_success() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let cities: Vec<Place> = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(cities))
}