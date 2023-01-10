extern crate rand; //importa lo de las dependencias (Cargo.toml)

use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {
    let _numero_secreto = rand::thread_rng().gen_range(1,101);
    println!("Adivina el numero secreto! (entre 1 y 100)");
    //println!("por motivos de debug el num secreto es: {}", _numero_secreto);
    let mut intentos:u8 = 1;
    
    loop {
	println!("Introduce un numero entero entre 1 y 100: ");
	//Debe ser declarado aqui para mutarse bien.
	let mut entrada = String::new();

	//recibir dato
	io::stdin().read_line(&mut entrada)
	    .expect("Error al leer la linea");
	//convertirlo a numero
	let entrada: u8 = match entrada.trim().parse(){
	    Ok(num) => num,
	    Err(_) => {
		println!("{} no es valido (prueba con un numero entero entre 1 y 100) ", entrada.trim());
		continue;
	    },
	};

	
	//comparar entrada, luego del parse, con numero secreto.
	match entrada.cmp(&_numero_secreto){
	    Ordering::Less     => {println!("\n{} es más pequeño\n", entrada);
				   intentos+=1;
	    },
	    Ordering::Greater  => {println!("\n{} es más grande\n", entrada);
				   intentos+=1;
	    },
	    Ordering::Equal    => {
		println!("\n==================================== \n ¡Le atinaste mamón! ¡Enhorabuena!");
		println!(" Te ha tomado [{}] intentos lograrlo \n====================================", intentos);
		break;
	    },
	}
    }
}
