use std::io;

fn main() {
    println!("Introduce un numero (entre 1 y 100) para obtener su serie fibonacci:");
    let cantidad: u8 = recibir_dato().trim().parse()
        .expect( " Error al recibir el numero. ");

    fibonacci(cantidad);
}

fn fibonacci(var: u8){
    println!("elegiste {}", var);
    if var < 102{
        let (mut temporal, mut anterior, mut valor): (u128, u128, u128) = (1, 1, 0);
        let mut x:u8 = 1;

        while x < var {
            print!("{}, ", valor); 
            valor = temporal; 
            temporal = anterior;
            anterior += valor; 
            x+=1;
        }
        println!("{}.",valor);
    } else {
        println!("el resultado será gigante D:");
    }
}

fn recibir_dato() -> String{            
    let mut var = String::new();
    io::stdin().read_line(&mut var)
        .expect(" Error al leer entrada ");
    return var;
}
