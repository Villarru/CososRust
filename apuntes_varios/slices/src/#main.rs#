fn main() {
    //los slice son un tipo de dato similar a un puntero, representa un pedazo
    // en una matriz o una cadena de caracteres.

    let arreglo = [1,88,3,37,5,6,24,25,26,99,12,42];
    let tro: &[i32] = &arreglo[1..7];
    
    println!("Slice original:");
    imprimir_slice(tro);
    println!("");
    
    let cadena = String::from("Hola olita jelou");
    println!("A ver que sale: {}", primera_palabra(&cadena));

    let holas = &cadena[5..11];
    println!("\n{}", &holas);
}

fn primera_palabra(c: &String)-> &str {
    let bytes = c.as_bytes();

    for (i, &coso) in bytes.iter().enumerate() {
        if coso == b' '{
            return &c[0..i];
        }
    }
    
    &c[..]
}


fn imprimir_slice(slice: &[i32]){
    for num in slice{
        print!("[{}] ", num);
    }
}
