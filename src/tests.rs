use super::feed::*;
use super::query::*;

#[test]
fn test_city() {
    // Test demo City output
    let a = Aq::new("demo").city("Shanghai").get();
    assert_eq!(a.city, r#"Shanghai"#);
}

#[test]
#[should_panic(expected = "Can not dupplicate with")]
fn test_duplicate() {
    // duplicate request test
    let a = Aq::new("demo")
        .city("Shanghai")
        .geo(121.4489017, 31.2047372)
        .get();
    assert_eq!(a.city, r#"Shanghai"#);
}

#[test]
fn test_query() {
    // Test demo City output
    let mut a = Queries::query("demo", "1,2,3,4");
    let vec = a.result.pop().unwrap();
    assert_eq!(vec.lat, 39.538047f64);
    assert_eq!(vec.lon, 116.683752f64);
    assert_eq!(vec.uid, 1463);
    assert_eq!(vec.aqi, "53");
}