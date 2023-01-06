use std::io;

fn main() {
    const PI: f64 = 3.1415926535;
    println!("Introduce radio de círculo: ");
    let mut radio = String::new();

    io::stdin().read_line(&mut radio)
        .expect(" Error al recibir radio ");

    let radio: f64 = radio.trim().parse()
        .expect(" Error al convertir el dato introducido a numero ");

    println!("El area de un circulo con radio {} es igual a {}", &radio, ((&radio*&radio)*PI) )
}
