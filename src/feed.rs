use std::io::Read;
use serde_json;
use serde_json::Value;
use reqwest;

pub struct Aq {
    token: String,
    feedtype: i8,

    lat: Option<f32>,
    lng: Option<f32>,

    city: Option<String>
}

pub struct AqFeed {
    pub city: String,
    pub aqi: f64,
    pub aqilevel: f64,
    pub co: f64,
    pub dew: f64,
    pub humidity: f64,
    pub no2: f64,
    pub o3: f64,
    pub pressure: f64,
    pub pm10: f64,
    pub pm25: f64,
    pub so2: f64,
    pub temperture: f64,
    pub wind: f64,
    pub wd: f64,
    pub time: String,
}

#[allow(dead_code)]
impl Aq { 
     pub fn new(token: &str) -> Aq {
        Aq { 
            token: token.to_string(),
            feedtype: 0,
            
            lat: None,
            lng: None,

            city: None,
        }
    }

    pub fn geo(&mut self, lat: f32, lng: f32) -> &mut Aq {
        
        self.feedtype = check(self.feedtype, 1);
        self.lat = Some(lat);
        self.lng = Some(lng);
        self
    }

    pub fn here(&mut self) -> &mut Aq {
        self.feedtype = check(self.feedtype, 2);
        self
    }

    pub fn city(&mut self, city: &str) -> &mut Aq{
        self.feedtype = check(self.feedtype, 3);
        self.city = Some(city.to_string());
        self
    }



    pub fn get(&self) -> AqFeed {
        if self.feedtype == 0 { panic!("should be select method to get Air Quality") }
       
        let feedtype: String = match self.feedtype {
            1 => "geo:".to_string() + &self.lat.unwrap().to_string() + &";".to_string() + &self.lng.unwrap().to_string(),
            2 => "here".to_string(),
            3 => self.city.clone().unwrap().to_string(),
            _ => panic!("wait, what?")
        };
        let url: String = "https://api.waqi.info/feed/".to_string()
                          + &feedtype.to_string()
                          + &"/?token=".to_string()
                          + &self.token.to_string();
        let mut r = reqwest::get(&url).unwrap();
        let mut d = String::new();
        r.read_to_string(&mut d).unwrap();

        let j: Value = serde_json::from_str(&d).unwrap();
        let time: String = remove_quotes(format!("{}{}", j["data"]["time"]["s"].to_string(), j["data"]["time"]["tz"].to_string()).as_str());
        let aqi = j["data"]["aqi"].as_f64().unwrap_or_default();
        let aqilevel = aqi_level(aqi);

        AqFeed {
            city: remove_quotes(j["data"]["city"]["name"].as_str().unwrap()),
            aqi: aqi,
            aqilevel: aqilevel,
            co: j["data"]["iaqi"]["co"]["v"].as_f64().unwrap_or_default(),
            dew: j["data"]["iaqi"]["d"]["v"].as_f64().unwrap_or_default(),
            humidity: j["data"]["iaqi"]["h"]["v"].as_f64().unwrap_or_default(),
            no2: j["data"]["iaqi"]["no2"]["v"].as_f64().unwrap_or_default(),
            o3: j["data"]["iaqi"]["o3"]["v"].as_f64().unwrap_or_default(),
            pressure: j["data"]["iaqi"]["p"]["v"].as_f64().unwrap_or_default(),
            pm10: j["data"]["iaqi"]["pm10"]["v"].as_f64().unwrap_or_default(),
            pm25: j["data"]["iaqi"]["pm25"]["v"].as_f64().unwrap_or_default(),
            so2: j["data"]["iaqi"]["so2"]["v"].as_f64().unwrap_or_default(),
            temperture: j["data"]["iaqi"]["t"]["v"].as_f64().unwrap_or_default(),
            wind: j["data"]["iaqi"]["w"]["v"].as_f64().unwrap_or_default(),
            wd: j["data"]["iaqi"]["wd"]["v"].as_f64().unwrap_or_default(),
            time: time.to_string(), 
        }
        
    }

}

fn check(s: i8, r: i8) -> i8 {
    if r < 0 { panic!("feedType should not be under 0") }
    if r > 3 { panic!("feedType Should not be over 3") }
    
    match (s, r) {
        (0, _) => r,
        (_, _) => panic!("Already feedType allocated, Can Not dupplicate 'geo()', 'here()', 'city()'.  try '.get()'")
    }
}

fn remove_quotes(s: &str) -> String {
    s.replace(r#"""#,"")
}


fn aqi_level(f: f64) -> f64 {
    let r: f64= if f >= 0f64   && f < 51f64  { 0f64 }
           else if f >= 51f64  && f < 101f64 { 1f64 }
           else if f >= 101f64 && f < 151f64 { 2f64 }
           else if f >= 151f64 && f < 201f64 { 3f64 }
           else if f >= 201f64 && f < 301f64 { 4f64 }
           else if f >  300f64               { 5f64 }
           else { panic!("Impossible input"); };

    r
}