use std::io;

fn ingresar_dato() -> f64{
    let mut dato = String::new();
    io::stdin().read_line(&mut dato).expect(" error al recibir dato ");
    let num:f64 = dato.trim().parse().expect(" error al convertir ");
    num
}

fn main() {
    println!("=== Promedio de 3 cantidades ===");
    println!("Intresa el primer numero: ");
    let mut res = ingresar_dato();

    println!("Intresa segundo numero: ");
    res += ingresar_dato();
    
    println!("Intresa el tercer numero: ");
    res += ingresar_dato();
    
    println!("Intresa cuarto numero: ");
    res += ingresar_dato();
    
    println!("El primedio es: {}", res/4.0);
}
