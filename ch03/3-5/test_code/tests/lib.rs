use test_code::add;

#[test]
fn test_add_ignored() {
    assert_eq!(-2, add(-1, -1));
}
