struct GenericVal<T> {
    gen_val : T,
}

impl<T> GenericVal<T> {
    fn val(&self) -> &T{
        &self.gen_val
    }
}

fn main(){
    let y = GenericVal {
        gen_val: 25
    };
    println!("{}",y.val());
}