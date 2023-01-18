use std::convert::From;

#[derive(Debug)]
struct Numero{
    value: i32,
}

impl From<i32> for Numero{
    fn from(cosa: i32) -> Self{
        Numero {value: cosa}
    }
}

fn main() {
    let num = Numero::from(32);
    // Tambien se puede hacer con into.
    let x = 34i32;
    let num2: Numero = x.into();
    println!("num = {:?}", num);
    println!("num2 = {:?}", num2);
}
