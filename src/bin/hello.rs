use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n", self.0, self.1)?;
        write!(f, "({} {})\n", self.2, self.3)
    }
}

fn main(){
    println!("hello world");
    println!("Hello {}","omprakash");
    println!("hello {name}",name="Lol");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    let pi = 3.141592;
    println!("Hello {0} is {1:.3}", "pi", pi);
    let tuple1 = (1, false, "cool");
    println!("{} {} {}", tuple1.0, tuple1.1, tuple1.2);
    let (a,b,c) = tuple1;
    println!("{} {} {}", tuple1.0, tuple1.1, tuple1.2);
    println!("{} {} {}", a,b,c);
    let matrix = Matrix(1.1,2.2,3.3,4.4);
    println!("{}",matrix);
    let xs = [1,2,3,4,5];
    println!("xs Array occupies {}",std::mem::size_of_val(&xs));
}

//hello
/* 
block
*/