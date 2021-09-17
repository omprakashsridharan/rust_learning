fn main(){
    let reference = &4;
    match reference {
        // Destructuring a reference
        &val => println!("Got a value via destructuring {:?}",val)
    }

    // Dereferencing a reference
    match *reference {
        val => println!("Got a value via dereferencing {:?}", val)
    }

    // Same as let reference = &3;
    let ref _is_a_reference = 3;

    let value = 5;
    match value {
        ref r => println!("Got a reference to a value {:?}", r),
    }


    let mut mut_value = 6;
    match mut_value {
        ref mut r => {
            println!("Got a mutable reference to value {:?}", r);
            *r +=1;
        }
    }
    println!("mut_value is now {}",mut_value);
}