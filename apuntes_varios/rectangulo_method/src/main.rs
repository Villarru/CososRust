#[derive(Debug)]
struct Rectangulo {
    alto: u32,
    largo: u32,
}
// ahi va el metodo:
impl Rectangulo {
    fn area(&self) -> u32 {
        self.alto * self.largo
    }
}


fn main() {
    let rect1 = Rectangulo{
        alto: 66,
        largo: 99,
    };

    
    println!("Helou uncle");
    // Cuidado con hacer bien la referencia.
    dbg!(&rect1);

    // Cada día más estrucural ;)
    println!("\n Area: {}", rect1.area());
}

