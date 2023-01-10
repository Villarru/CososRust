use std::io;
fn main() {
    println!("Introduce cantidad de kilogramos: ");
    let mut dato = String::new();
    io::stdin().read_line(&mut dato).expect(" error al recibir kilogramos ");

    let dato: f32 = dato.trim().parse().expect(" error al convertir entrada a numero ");

    let libras: f32 = dato*2.2046;
    println!("{} kilogramos equivalen a {} libras.", &dato, &libras);
}
