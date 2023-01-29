// Usar enum, struct y fn para poder procesar información de automoviles.
extern crate rand;

use rand::Rng;

struct Auto {
    color: String,
    motor: Transmision,
    techo: bool,
    estado: (Estado, u32),
}
#[derive(PartialEq, Debug)]
enum Transmision {
    Manual,
    SemiAuto,
    Automatica,
}
#[derive(PartialEq, Debug)]
enum Estado {
    Nuevo,
    Usado,
}

fn calidad(kilometraje: u32) -> (Estado, u32) {
    if kilometraje > 0 {
        return (Estado::Usado, kilometraje);
    }
    (Estado::Nuevo, kilometraje)
}

fn fabricar_auto(x: &u32) {
    let y: u32 = rand::thread_rng().gen_range(1, 4);
    let z: u32 = rand::thread_rng().gen_range(1, 4);
    let mut color = String::from("Negro");
    let mut motor = Transmision::Manual;
    let mut kilometraje: u32 = 0;
    let mut techo = true;

    // Condiciones aleatoreas, habrá que mejorar esto en futuras versiones

    if x % y == 0 {
        color = String::from("Naranja");
        kilometraje = 0;
        techo = false;
    } else {
        if x % 3 == 0 {
            color = String::from("Verde");
            techo = false;
            motor = Transmision::Automatica;
            if (x + &y) > (8 - z) {
                kilometraje = x * y * 276
            }
        }
    }
    if y % 3 == 0 {
        color = String::from("Azul");
        techo = false;
        motor = Transmision::Automatica;
    } else {
        if y % 2 != 0 {
            kilometraje = 0;
            color = String::from("Verde");
            motor = Transmision::SemiAuto;
        }
        if x > &5u32 {
            kilometraje = x + y * x * 4 * z;
        }
    }
    if x + &y > z && x % 2 == 0 {
        color = String::from("Blanco");
        if x & 3 == 0 {
            color = String::from("Negro");
            techo = false;
            motor = Transmision::Automatica;
        }
    } else {
        if x + &z > y {
            motor = Transmision::SemiAuto;
            if y % 2 == 0 {
                techo = true;
                color = String::from("Violeta");
                kilometraje = x + 132 * y;
            }
        }
    }
    if y == 13 {
        color = String::from("Turqueza");
        techo = true;
        kilometraje = 0;
    }

    // Fabricación del auto
    let carro = Auto {
        color,
        motor,
        techo,
        estado: calidad(kilometraje),
    };
    imprimir_info(&carro, x);
}

fn imprimir_info(carro: &Auto, num: &u32) {
    let mut techo = String::from("");
    if carro.techo {
        techo = String::from("Es convertible\n ");
    }
    let (estado, km): &(Estado, u32) = &carro.estado;

    hr();
    println!(
        " --- Auto {} --- \n Color: {}\n Motor: {:?}\n {}Estado {:?}\n Kilometraje: {}",
        num, &carro.color, &carro.motor, techo, estado, km
    );
}

fn hr() {
    println!("=============================");
}

fn main() {
    let mut cantidad = 0;
    loop {
        cantidad += 1;
        fabricar_auto(&cantidad);
        if cantidad >= 8 {
            break;
        }
    }
    hr();
}
