use std::io;
fn main() {
    println!("Introduce centímetros: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect(" error en la entrada de datos ");

    let cm: f32 = entrada.trim().parse().expect(" error al convertir a numero ");
    
    println!("{} cm equivale a {} pulgadas ", &entrada.trim(), cm/2.56);
}
