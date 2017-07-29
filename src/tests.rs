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
    let mut a = Queries::query("demo", "39.379436,116.091230,40.235643,116.784382");
    let vec = a.data.pop().unwrap();
    assert_eq!((vec.lat / 1f64) as i64, 39);
    assert_eq!((vec.lon / 1f64) as i64, 116);
}