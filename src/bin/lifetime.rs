fn main() {
    let string1 = String::from("abcd");
    let string2 = "中文輸入";
    // String will automatically transform to str when calling a method, but you can point out it explicitly
    println!("{}", longest(&string1.as_str(), &string2));

    let novel = String::from("Call me Ishmael. Some Years ago...");
    let first_sentence = novel.split('.').next().expect("Could not fount a '.'");
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 生命週期省略的三個規則
// 1. 輸入生命週期：每個引用類型都有自己的生命週期
// 2. 輸出生命週期：如果只有1個輸入生命週期，那麼該生命週期會被賦值給所有的輸出生命週期
// 3. 輸出生命週期：如果方法有多個輸入生命週期，但是其中有一個是&self或者&mut self，那麼self的生命週期會被賦值給所有的輸出生命週期
// 當編譯器已經應用了所有規則荏苒無法確定所有引用的生命週期，就會報錯
