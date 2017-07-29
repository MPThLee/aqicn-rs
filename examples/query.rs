extern crate aqicn;

fn main() {
    let mut a = aqicn::query::Queries::query("demo", "39.379436,116.091230,40.235643,116.784382");
    let vec = a.data.pop().unwrap();
    println!("{}", vec.lat);
    println!("{}", vec.lon);
    println!("{}", vec.uid);
    println!("{}", vec.aqi);
}