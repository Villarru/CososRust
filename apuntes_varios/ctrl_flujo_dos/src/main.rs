fn main() {
    //los loop pueden ir aninados y break y continue sirven normal de adentro
    // hacia afuera. pero si quieres parar un loop exterior desde un loop
    // interior puedes crear loop label.
    lupapu();

    //luego el while de toda la vidaL
    let mut acumulado: u32 = 12;
    let mut cuenta: u32 = 0;

    println!("\nAcumulado: {}, cuenta: {}\n while:", acumulado, cuenta);

    while acumulado < 80{
        cuenta += 1;
        acumulado += 1;
    }

    println!("Acumulado: {}, se le sumaron: {}\n", acumulado, cuenta);

    //por ultimo los for son mas usados como el for in normalon.
    let arreglo = [1, 3, 5, 7, 9, 11, 15, 17, 19];
    for coso in arreglo{
        println!(": : : : : : : : : : : : : {coso}");
    }

    // y por rangos como en python.
    println!("\nLista impresa con for num in (0..5).rev()");
    for numerito in (0..5).rev(){
        println!("{numerito}");
    }
}

fn lupapu(){
    let mut cuenta: i16 = 0;
    
    'basta_de_contar: loop{
        println!("contador: {}", cuenta);
        let mut restante: i16 = 13;

        loop{
            println!("restante: {}", restante);
            if restante == 11 {
                break;
            }
            if cuenta == 3{
                break 'basta_de_contar;
            }
            restante -=1;
        }
        cuenta +=1;
    }
    println!("\ncontador final = {}", cuenta);
}


