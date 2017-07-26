use super::feed::*;

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
