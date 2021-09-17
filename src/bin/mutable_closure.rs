fn main(){
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("Count is {}",count);
    };

    inc();
    inc();
    // After this count can be mutably borrowed 

    
}