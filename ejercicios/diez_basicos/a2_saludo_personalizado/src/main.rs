use std::io;

fn main() {
    println!("Introduce tu nombre: ");
    let mut nombre = String::new();

    io::stdin().read_line(&mut nombre).
        expect(" no se ha podido recibir dato");

    println!("Un saludo a {}y otro saludo a san vicente", nombre);
}
