use std::io;
// Sexta revisión:
fn main() {
    println!("=========================================================");
    println!("==== Convertidor de Fahrenheit a celsius y viceversa ====");
    println!("=========================================================");
 
    let mut eleccion: u8 = 4;

    while eleccion != 0{
        println!("\nElije: 1. fahrenheit a celsius.  2. celsius a fahrenheit. 0.Cerrar");

        eleccion = match recibir_dato().trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if validar_eleccion(eleccion) {
            'convertir: loop{

                let cantidad: f32 = match recibir_dato().trim().parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                };

                convertir_segun_eleccion(eleccion, cantidad);
                break 'convertir;
            }
        }
    }
}

fn recibir_dato() -> String{            
    let mut var = String::new();
    io::stdin().read_line(&mut var)
        .expect(" Error al leer entrada ");
    return var;
}

fn validar_eleccion(var: u8) -> bool{
    if var == 1 {
        println!("Introduce grados Fahrenheit: ");
        return true;
    } else if var == 2 {
        println!("Introduce grados Celsius: ");
        return true;
    } else if var !=0{
        println!("{} no es una opción en el menu.", var);
        return false;
    } else {
        return false;
    }
}

fn convertir_segun_eleccion(valor: u8, cantidad: f32){
    const UNO_PUNTO_OCHO: f32 = 1.8;
    if valor == 1 {
        println!("El valor {} grados fahrengeit equivale a {} celsius.", cantidad, ((cantidad-32.0)/UNO_PUNTO_OCHO) );
    } else {
        println!("El valor {} grados celsius equivale a {} grados fahrengeit.", cantidad, (cantidad*UNO_PUNTO_OCHO + 32.0) );
    }
}
