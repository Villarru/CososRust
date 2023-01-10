use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Introduce tu edad (años): ");

    io::stdin().read_line(&mut entrada).expect(" error al leer edad ");

    let x: u64= entrada.trim().parse().expect(" error al convertir a u8 ");
    let f: f32 = x as f32;

    println!("Tu tiempo vivido puesto en otros formatos es: ");
    println!("{} décadas, {} lustros, {} meses, {} semanas\nDías: {}, Horas: {}, Minutos {}",
             &f/10.0, &f/5.0, x*12, x*52, x*365, x*364*24, x*364*24*60);
    // oh no, números mágicos. c; menos mal que se entiende.
}
