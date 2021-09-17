fn func<T: std::fmt::Display>(t: T){
    println!("Hello {}",t);
}

fn main(){
    func(24);
}