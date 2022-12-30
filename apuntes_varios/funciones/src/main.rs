use std::io;
fn main() {
    println!("Introduce numero");

    let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("");

    let num: i32 = num.trim().parse()
        .expect("eso no es un numero xd\n"); 
        
    //llamadas:
    imprimir_numero(num);
    sumar_dos_num(num, num);
    println!("la resta de {} menos 5 da: {}", num, restar_dos_num(num,5));

    //divergente();
    //las funciones divergentes pueden usarse como cualquier tipo
    //let asdf: i32 = divergente();
    //let asdf: String = divergente();

    //funcion con let, me recuerda JS.
    // let func: fn(i32) -> i32;
    //f es un binding que apunta a una funcion que recibe un i32 y regresa otro
    // con inferencia de tipos
    // let func = sumar_uno;
    // sin inferencia
    let func: fn(i32) -> i32 = sumar_uno;
    println!("El resultado de sumar uno con la func apuntada: {}", func(num));
}

//este es para explicar el binding de una funciopn.
fn sumar_uno(i: i32) -> i32 {
   i+1
}

//aca la definicion de tipo es como con let.
fn imprimir_numero(x: i32){
    println!("El numero es: {}", x);
}

fn sumar_dos_num(x: i32, y: i32){
    println!("La suma de {} y {} es: {}", x, y, x + y);
}
 
//para retornar valores se usa una flecha y el tipo de dato retornado
fn restar_dos_num(x: i32, y: i32) -> i32 {
    x - y
}

//funcion divergente

fn divergente() -> ! {
    panic!("Esta funcion no retorna");
}


