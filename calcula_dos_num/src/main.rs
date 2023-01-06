use std::io;

fn recibir_numero() -> f32{            
    let mut var = String::new();
    io::stdin().read_line(&mut var)
        .expect(" Error al leer entrada ");

    let x: f32 = var.trim().parse().
        expect(" Error al recibir el numero ");

    return x;
}

fn calcular_segun_op(op: i32, x: &f32, y: &f32) -> bool{
    let algo: f32 = 0.0;
    println!("");
    if op == 1 {
        println!("{} + {} = {}", x, y, x+y);
        return false;
    }else if op == 2 {
        println!("{} - {} = {}", x, y, x-y);
        return false;
    }else if op == 3 {
        println!("{} x {} = {}", x, y, x*y);
        return false;
    }else if op == 4 {
        if y != &algo {
            println!("{} / {} = {}", x, y, x/y);
        }else {
            println!("No puedo dividir entre 0\n");
        } 
        return false;
    }else{
        println!("La opcición {} no esta en el menu.", op);
        return true;
    }
}

fn main() {
    loop{
        println!("Introduce primer numero: ");
        let num1: f32 = recibir_numero();
        println!("Introduce segundo numero: ");
        let num2: f32 = recibir_numero();
        
        println!("\nHas introducido: {} y {}\n", num1, num2);
        loop{
            println!("Introduce el numero de la operación deseada: \n1.SUMA 2.RESTA 3.MULTIPLICACION 4.DIVISION");
            let opcion = recibir_numero() as i32;
            if calcular_segun_op(opcion, &num1, &num2){continue;}
            break;
        }
        println!("\n¿Introducir otros numeros? 1.SI 2.NO");
        let cerrar = recibir_numero() as i32;
        if cerrar == 1{continue;}
        break;
    }
}
