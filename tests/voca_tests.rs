use voca as v;

#[test]
fn test_lower_case() {
    let text = "Goo";
    assert_eq!(v::lower_case("Good"), "good")
}
