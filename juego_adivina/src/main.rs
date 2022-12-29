extern crate rand; //importa lo de las dependencias (Cargo.toml)

use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {
    let _numero_secreto = rand::thread_rng().gen_range(1,101);
    println!("Adivina el numero!");
    //println!("por motivos de debug el num secreto es: {}", _numero_secreto);
    let mut intentos:u32 = 1;
    
    loop {
	println!("Introduce un numero: ");
	//Debe ser declarado aqui para mutarse bien.
	let mut entrada = String::new();

	//recibir dato
	io::stdin().read_line(&mut entrada)
	    .expect("Error al leer la linea");
	//convertirlo a numero
	let entrada: u32 = match entrada.trim().parse(){
	    Ok(num) => num,
	    Err(_) => {
		println!("{} no es un numero", entrada.trim());
		continue;
	    },
	};

	
	//comparar entrada, luego del parse, con numero secreto.
	match entrada.cmp(&_numero_secreto){
	    Ordering::Less     => {println!("\nEl número {} es pequeño\n", entrada);
				   intentos+=1;
	    },
	    Ordering::Greater  => {println!("\nEl número {} es grande\n", entrada);
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
