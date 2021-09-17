fn main(){
    let s = String::from("Hello");
    let print = || println!("Color is {}",s);
    print();
    println!("Color after closure called is {}",s);
    let _color_moved = s;
    // print() cannot be called here
    //print();
}