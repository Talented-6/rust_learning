fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // pattern matching
    // &list[1..]
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let list = [1, 2, 3, 4, 5500, 60000];
    println!("{}", largest(&list));
}
