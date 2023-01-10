use std::io;
fn main() {
    println!("Introduce cantidad de galones(E.E.U.U líquido) a converitr a litros: ");
    let mut cantidad = String::new();
    io::stdin().read_line(&mut cantidad)
        .expect(" error al recibir cantidad de galones ");

    let gal: f32 = cantidad.trim().parse()
        .expect(" error al convertir entrada a numero ");

    let litros: f32 = gal*3.78541;
    println!("{} galones es igual a {} litros.", &gal, &litros);
}
