// fn larget<T:std::cmp::PartialOrd>(list:&[T]) -> T{

// }

struct Point<T, U> {
    x: T,
    y: U,
}
impl<A, B> Point<A, B> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<A, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: 4, y: "big" };
    let p3 = p1.mixup(p2);
    println!("p3.x={} p3.y={}", p3.x, p3.y);
}
// because of monomorphization(单态化), the code with generics is as fast as normal codes
