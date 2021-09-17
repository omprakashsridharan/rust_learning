use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    breadth: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.breadth
    }
}

fn area<T: HasArea>(t: T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 2.0,
        breadth: 3.4,
    };
    println!("{}", area(rectangle))
}
