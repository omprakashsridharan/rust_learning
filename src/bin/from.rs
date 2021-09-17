use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number {value}
    }
}

fn main(){
    let num = Number::from(30);
    println!("Number struct is {:?}",num);

    //INTO
    let int = 5;
    let num1: Number = int.into();
    println!("Num1 is {:?}", num1);
}