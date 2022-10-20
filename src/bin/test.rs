// arrange - act - assert
// text attribute

#[test]
#[should_panic(expected = "Guess value should be less than 100 annd more than 1")]
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
    // String与&str
    let test = "999";
    println!("{} {}", test.to_string(), String::from(test));
    // 数的转换大到小会截断
    let a = 3222;
    println!("{}", a as u8);
    // 计算实际字符数
    let slice2 = "안녕!你好!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
    // "_"
    let num = 1___________0_______________________9_u8;
    println!("{}", num);
    // raw string
    println!(r#"C:/Program Files/Adobe"#);
    // print(a char needs at most 4 bytes to store)
    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    println!("The smallest i8 is {} and the biggest i8 is {}.", i8::MIN, i8::MAX); // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
    println!("The smallest u8 is {} and the biggest u8 is {}.", u8::MIN, u8::MAX);
    println!("The smallest i16 is {} and the biggest i16 is {}.", i16::MIN, i16::MAX);
    println!("The smallest u16 is {} and the biggest u16 is {}.", u16::MIN, u16::MAX);
    println!("The smallest i32 is {} and the biggest i32 is {}.", i32::MIN, i32::MAX);
    println!("The smallest u32 is {} and the biggest u32 is {}.", u32::MIN, u32::MAX);
    println!("The smallest i64 is {} and the biggest i64 is {}.", i64::MIN, i64::MAX);
    println!("The smallest u64 is {} and the biggest u64 is {}.", u64::MIN, u64::MAX);
    println!("The smallest i128 is {} and the biggest i128 is {}.", i128::MIN, i128::MAX);
    println!("The smallest u128 is {} and the biggest u128 is {}.", u128::MIN, u128::MAX);

    // size
    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));

    // into
    let test: String = "test".into();
}
