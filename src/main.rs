use rand::Rng; // trait类似接口
use std::cmp::Ordering;
use std::io; // prelude预导入，不需要导入

// 定义常量
// const MAX_POINTS: u32 = 100_000;

fn main() {
    //小游戏
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字是{}", secret_number);
    loop {
        println!("猜测一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行"); // io::Result Ok,Err
        print!("你猜测的数是：{}", guess); // guess里已经包含回车了
        // let guess: u32 = guess.trim().parse().expect("转换失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
    // 基础数据类型（标量，存储在栈上）
    // 元组
    let tup = (1.0f32, b'a', '中');
    let (x, y, z) = tup;
    println!("{}, {}, {}, ", tup.0, tup.1, tup.2);
    println!("{}, {}, {}, ", x, y, z);
    // 数组（存放在栈上而不是堆上）
    let a = [1, 2, 3, 4, 5];
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    for element in a.iter() {
        print!("{}\t", element);
    }
    print!("\n");
    for element in (1..4).rev() {
        print!("{}\t", element);
    }
    print!("\n");
    // 字符串（存储在堆上的）
    let mut s = String::from("你好，世界");
    s.push_str("Hello, world!");
    let s1 = s;
    let s2 = s1.clone();
}

fn another_function() {
    println!("Hello, World");
}
