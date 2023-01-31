use std::io;

fn main() {
    println!("Introduce pesos mxn: ");
    let mut p = String::new();

    io::stdin().read_line(&mut p).
        expect(" no se ha podido recibir dato");

    let peje_coin: f64 = p.trim().parse().expect("");
    println!("{} pesos son: \n{} dolares\n{} euros\n{} pesos argentinos\n{} bolibares",
             peje_coin, peje_coin*0.052,peje_coin*0.049, peje_coin*9.256, peje_coin*12855.524);
}
