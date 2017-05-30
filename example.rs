extern crate aqicn;

extern crate hyper;
extern crate serde_json;
extern crate regex;

fn main() {
    let s = aqicn::feed::Aq::new("demo").here().get();
    println!("City Name  : {}", s.city);
    println!("AQI        : {}", s.aqi);
    println!("AQI Level  : {}", s.aqilevel);
    println!("CO         : {}", s.co);
    println!("Dew        : {}", s.dew);
    println!("Humidity   : {}", s.humidity);
    println!("No2        : {}", s.no2);
    println!("O3         : {}", s.o3);
    println!("Pressure   : {}", s.pressure);
    println!("PM10       : {}", s.pm10);
    println!("PM2.5      : {}", s.pm25);
    println!("SO2        : {}", s.so2);
    println!("Temperture : {}", s.temperture);
    println!("Wind Speed : {}", s.wind);
    println!("WD         : {}", s.wd);
    println!("Time       : {}", s.time);
}
