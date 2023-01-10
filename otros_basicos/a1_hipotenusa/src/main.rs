use std::io;

fn hipotenusa(x: f64, y: f64)-> f64{
    let hip: f64 = (x*x) + (y*y);
    hip.powf(1.0/2.0)
}

fn recibir_dato() -> String{
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect(" error al leer dato ");
    valor
}

fn main() {
    println!("=== calcular hipotenusa ===");
    
    println!("Introduce primer cateto: ");
    let cateto1: f64 = recibir_dato().
        trim().parse().expect(" error al converitr");
    
    println!("Introduce segundo cateto:");
    let cateto2: f64 = recibir_dato().
        trim().parse().expect(" error al converitr");

    println!("El resultado es: {} ", hipotenusa(cateto1, cateto2));
}
