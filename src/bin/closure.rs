fn main(){
    fn func(i: i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32| -> i32 { i+1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("Function: {}", func(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
}