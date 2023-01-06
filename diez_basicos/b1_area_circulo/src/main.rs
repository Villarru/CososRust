use std::io;

fn main() {
    println!("Introduce el radio del circulo: ");
    let mut r = String::new();
    const PI: f32 = 3.14159265;
    
    io::stdin().read_line(&mut r).
        expect(" no se ha podido recibir dato");

    let r: f32 = r.trim().parse().expect(" error al convertir a numero ");

    println!("El área de un círculo con radio {} es de {}", r, (r*r)*PI);
}
