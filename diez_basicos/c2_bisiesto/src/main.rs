use std::io;

fn main() {
    println!("Introduce un año: ");
    let mut a = String::new();

    io::stdin().read_line(&mut a).
        expect(" no se ha podido recibir dato");

    let a: i32 = a.trim().parse().expect(" error al convertir ");

    if a%4 == 0{
        if a%100 == 0{
            if a%400 == 0{
                println!("{}  es bisiesto", a);
            } else {
                println!("{} no es bisiesto", a);
            }
        } else{
            println!("{}  es bisiesto", a);
        }
    }else{
        println!("{} no es bisiesto", a);
    }
}
