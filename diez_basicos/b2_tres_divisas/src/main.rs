use std::io;

fn main() {
    
    println!("Introduce pesos (mxn 2022) y te devolvere el equivalente en dolares, euros y bolivar soverano");

    let mut peje_coin = String::new();
    io::stdin().read_line(&mut peje_coin)
        .expect(" error al recibir cantidad en pesos");

    let peje_coin: f64 = peje_coin.trim().parse()
        .expect(" error al convertir a numero ");

    println!("{} pesos son:\n > {} dólares\n > {} euros\n > {} bolivares\n > {} pesos argentinos",
             peje_coin, peje_coin*0.052, peje_coin*0.049, peje_coin*12858.604, peje_coin*9.26);
    
    

}
