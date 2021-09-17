fn main(){
    // iter()
    let names = vec!["A","B","C"];
    for name in names.iter() {
        match name {
            &"A" => println!("Apple"),
            _ => println!("{}",name),
        }
    }
    println!("{:?}",names);

    //into_iter()
    for name in names.into_iter(){
        match name {
            "A" => println!("Moved apple"),
            _ => println!("Moved {}",name),
        }
    }
    //names is consumed and not available beyond this for loop
    // println!("{:?}",names)

    let mut marks = vec![1,2,3];

    for mark in marks.iter_mut(){
        *mark += 1;
    }
    println!("{:?}",marks)
}