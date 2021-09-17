#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main(){
    let p = Person {
        name: String::from("Om"),
        age: 24,
    };
    println!("p {:?}",p);
}