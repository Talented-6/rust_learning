use rand::Rng; // trait类似接口
use std::cmp::Ordering;
use std::collections::*;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write}; // prelude预导入，不需要导入 // 主要作用就是测试时和预导入模块

// 定义常量
// const MAX_POINTS: u32 k= 100_000;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}
fn main() {
    first_introduction();
    second_basis();
    third_more();
}

fn first_introduction() {
    //小游戏
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字是{}", secret_number);
    loop {
        println!("猜测一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行"); // io::Result Ok,Err
        print!("你猜测的数是：{}", guess); // guess里已经包含回车  了
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
}
fn second_basis() {
    // 基础数据类型（标量，存储在栈上）
    // 元组
    let tup = (1.0f32, b'a', '中');
    let (x, y, z) = tup;
    println!("{}, {}, {}, ", tup.0, tup.1, tup.2);
    println!("{}, {}, {}, ", x, y, z);
    println!("{}", b'8'); // 这个的类型是u8
    // println!("{}",b"8"); // 这个的类型是[u8;1]
    // 数组（存放在栈上而不是堆上）
    let _a = [1, 2, 3, 4, 5];
    let _a: [u32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    // let first = a[0];
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
    let _s2 = s1.clone();
    //结构体
    let rect = Rectangle {
        width: 100,
        length: 200,
    };
    println!("{} {:?} {:#?}", rect.area(), rect, rect);
}

fn third_more() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(110);
    println!("The third element is {}", &v[2]);
    match v.get(2) {
        Some(third) => println!("The third element is {}", &v[2]),
        None => println!("There is no third element"),
    }
}
