use std::io;
// Sexta revisión:
fn main() {
    println!("=========================================================");
    println!("==== Convertidor de Fahrenheit a celsius y viceversa ====");
    println!("=========================================================");
 
    let mut eleccion: i8 = 4;

    loop{
        println!("\nElije: 1. fahrenheit a celsius.  2. celsius a fahrenheit.");

        eleccion = match recibir_dato().trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        eleccion = validar_eleccion(eleccion);
        if eleccion == 3 {continue};
 
        'convertir: loop{

            let cantidad: f32 = match recibir_dato().trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            convertir_segun_eleccion(eleccion, cantidad);
            break 'convertir;
        }
        break;
    }
}

fn recibir_dato() -> String{            
    let mut var = String::new();
    io::stdin().read_line(&mut var)
        .expect(" Error al leer entrada ");
    return var;
}

fn validar_eleccion(var: i8) -> i8{
    if var == 1 {
        println!("Introduce grados Fahrenheit: ");
        return var;
    } else if var == 2 {
        println!("Introduce grados Celsius: ");
        return var;
    } else {
        println!("{} no es una opción en el menu.", var);
        return 3;
    }
}

fn convertir_segun_eleccion(valor: i8, cantidad: f32){
    const UNO_PUNTO_OCHO: f32 = 1.8;
    if valor == 1 {
        println!("El valor {} grados fahrengeit equivale a {} celsius.\n", cantidad, ((cantidad-32.0)/UNO_PUNTO_OCHO) );
    } else {
        println!("El valor {} grados celsius equivale a {} grados fahrengeit.\n", cantidad, (cantidad*UNO_PUNTO_OCHO + 32.0) );
    }
}
