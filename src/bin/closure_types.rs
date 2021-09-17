fn apply<F>(f: F) where
    F: FnOnce
    () {
        f();
    }

fn main(){
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!!");
        println!("Then i screamed {}",farewell);

        // std::mem::drop(farewell);
    };
    
    apply(diary);
    farewell.push_str("cool");
    println!("Value of farewell is {}",farewell);
}