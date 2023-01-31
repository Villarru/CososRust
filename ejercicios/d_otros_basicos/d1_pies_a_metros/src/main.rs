use std::io;
fn main() {
    println!("Introduce medida en pies: ");
    let mut medida = String::new();
    io::stdin().read_line(&mut medida).expect(" error al recibir medida ");

    let pies: f32 = medida.trim().parse().expect(" error al convertir entrada a numero ");

    let metros: f32 = pies/3.2808;
    println!("{} pies equivalen a {} metros.", &pies, &metros);
}
