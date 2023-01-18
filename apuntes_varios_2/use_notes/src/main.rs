// Pa que no salgan las alertas de código sin usar.
// #![allow(dead_code)]

enum Status{
    Rich,
    Poor,
}

enum Work{
    Civilian,
    Soldier,
}

fn main() {
    // Usas 'use' para usar las cosillas del enum sin tener que hacer todo el
    // show dentro del scope.
    use crate::Status::{Poor, Rich};
    // Lo mismo que lo anterior, pero usa todo lo del work en vez de una lista
    // finita de opciones.
    use crate::Work::*;

    let status = Poor;

    let work = Civilian;

    match status {
        Rich => println!("A perro, trais unos Jordan"),
        Poor => println!("Hola, me presento"),
    }

    match work{
        Civilian => println!("A trabajar, papus"),
        Soldier => println!("A luchar, camaradas"),
    }
    
    println!("fin del programa");
}
