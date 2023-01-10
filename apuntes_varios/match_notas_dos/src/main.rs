extern crate rand; 
use rand::Rng;

fn poner_sombrerito(){
    println!("Conseguiste sombrerito");
}
fn quitar_somberirto(){
    println!("Ahora tienes una deunda de sombrerito");
}
fn mover_jugador(x: i32){
    println!("Sacaste {}",x);
}

fn main() {
    let lanzar_dado = rand::thread_rng().gen_range(1,6);
    match lanzar_dado {
        3 => poner_sombrerito(),
        4 => quitar_somberirto(),
        other => mover_jugador(other),
        // _ => (),  // esto es para evitar tener que poner todos los casos del
        // 1 al 5, es como decir "else{}"
    }

    //El match es como un if, pero funciona bien con los enum, sin embargo es
    // posible usar enum con if let, para hacer codigo más legible, pero la
    // desventaja es que pierdes la exaustividad del match.

    let config_max = Some(5u8);
    match config_max {
        Some(max) => println!("Valor máximo es {}", max),
        _ => (),
    }

    // Lo anterior puede reescribirse como if let:
    let config_max = Some(5u8);
    if let Some (max) = config_max {
        println!("Valor maximo puesto {}", max);
    }
    
}
