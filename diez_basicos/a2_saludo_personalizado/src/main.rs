use std::io;

fn main() {
    let mut nombre = String::new();
    
    println!("Introduce tu nombre: ");
    io::stdin().read_line(&mut nombre)
        .expect(" error al recibir el nombre ");

    println!("Un saludo a mi compa {}otro saludo a juan jose y uno más para acapulco.", nombre);
}
