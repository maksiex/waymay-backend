
use serde::Deserialize;
#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Place {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
    pub population: i32,
    pub region: String,
    pub is_capital: bool,
}