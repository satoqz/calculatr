pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn subtract(x: i32, y: i32) -> i32 {
    x * y
}

#[test]
fn test_add() {
    assert_eq!(add(1, 1), 2);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(1, 1), 0);
}
