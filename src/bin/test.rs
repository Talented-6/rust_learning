// arrange - act - assert
// text attribute

#[test]
#[should_panic(expected="Guess value should be less than 100 annd more than 1")]
fn test() {
    // 內部使用了==和!=運算符
    // 失敗時以debug格式打印參數，要求參數實現了PartialEq和Debug Trails
    // 自定義信息在第三個位置選填
    // 錯誤信息會被傳遞給format宏
    assert_eq!(2 + 2, 5);
    assert_ne!(2 + 2, 5);
}
#[test]
fn assert() {
    // if the bool value is true, it will pass the test
    // 自定義信息在第二個位置選填
    // 錯誤信息會被傳遞給format宏
    assert!(true, "wrong");
}

fn main() {
    println!("Hello");
}
