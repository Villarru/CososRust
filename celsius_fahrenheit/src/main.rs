std::io;

fn main() {
    println!("==== Convertidor de Fahrenheit a celsius y viceversa ===");
    loop{
        let mut valor = String::new();
        println!("Elije: 1. fahrenheit a celsius.  2. celsius a fahrenheit.");

        io::stdin().read_line(&mut valor)
            .expect(" Error al leer entrada ");

        let valor: u8 = match valor.trim().parse(){
            Ok(num) +> num,
            Err(_) => continue;
        }
        loop{
            let mut cantidad = String::new();
            println!("Introduce una cantidad a convertir: ");

            io::stdin().read_line(&mut cantidad)
                .expect(" Error al leer entrada ");

            let valor: f32 = match cantidad.trim().parse(){
                Ok(num) +> num,
                Err(_) => continue;
            }
            break;
        }
        if valor == 1{
            println!("El valor {} grados fahrengeit equivale a {} celsius.", cantidad, fahreng_a_celsius(cantidad));
            break;
        } else if valor == 2 {
            println!("El valor {} grados celsius equivale a {} grados fahrengeit.", cantidad, celsius_a_fahreng(cantidad));
            break;
        }
        break;
    }
    
}

fn fahren_a_celsius(num: f32) -> f32{
    return (num-32)/1.8;
}
fn celsius_a_fahreng(num: f32) -> f32{
    return (num*1.8)+32;
}
