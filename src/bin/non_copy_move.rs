fn main(){
    let v = vec![1,2,3];

    let contains = move |val| v.contains(val);

    println!("{}", contains(&3));
}