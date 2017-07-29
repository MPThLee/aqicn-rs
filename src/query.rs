use serde_json;

#[derive(Deserialize, Debug)]
pub struct Queries {
    status: String,
    pub data: Vec<Query>
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub lat: f64,
    pub lon: f64,
    pub uid: i32,
    pub aqi: String
}

impl Queries {
    pub fn query(token: &str, latlng: &str) -> Queries {
        let url: String = "https://api.waqi.info/map/bounds/?latlng=".to_string() + &latlng.to_string() +
            &"&token=".to_string() + &token.to_string();
        let data = super::get::get_data(&url).unwrap();
        let de: Queries = serde_json::from_value(data).unwrap();
        de
    }
}