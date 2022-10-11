use rust_learning;

// tests目錄被cargo特殊對待，只有在cargo test下纔會運行
// cargo test --test integration_test
// tests目錄下每個文件都被編譯成單獨的crate

#[test]
fn it_adds_two() {
    assert_eq!(4, 4);
}
