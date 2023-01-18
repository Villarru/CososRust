#![allow(overflowing_literals)]
fn main() {
    println!("Veamos unos cast");
    let decimal: f32 = 3.523_f32;

    //aqui hay broncas falta el as
    //let num: u8 = entero;
    let entero = decimal as u8;
    let caracter = entero as char;
    // Esto no se puede pero ahi va;
    // let character = decimal as char;

    println!("Casteo: {} -> {} -> {:?}", decimal, entero, caracter);

    // Puede ser que un numero entero tipo u64 no quepa en un tipo u8 
    println!("1000 as u8= {}", 1000 as u8); //232

    // Igual que un numero negativo no coincida con un unsigned
    println!("-1 as u8= {}", (-1i8) as u8); //255

    println!("1000 modulo 256= {}", 1000 % 256); //232

    println!("128 as i8 es= {}\n", 128 as i8); // -128 es su complemento

    // Ahora con decimales
    // Desde rust 1.45, 'as' realiza un staturating cast, cuando hace punto
    // flotante a enteros. Hace redondeos quitandole el decimal, y  si el
    // numero sobrepasa el entero mas grande, te lo deja al maximo,y si el
    // numero sobrepasa el entero mas chico, te deja el minimo.
    println!("Todos estos son f32:");
    println!("300.0 as u8= {}", 300.0_f32 as u8);
    println!("101.45 as u8= {}", 101.45_f32 as u8);
    println!("21.85 as u8= {}", 21.85_f32 as u8);
    println!("-32.35 as i8= {}", (-32.35_f32) as i8);
    println!("-32.95 as i8= {}", (-32.95_f32) as i8);
    println!("-199.56 as i8= {}", (-199.56_f32) as i8);
    println!("-1.26 as u8= {}", (-1.26_f32) as u8);

    // si no es un numero, te lo deja en 0
    println!("nan as u8= {}\n", f32::NAN as u8);

    // pero hacerlo así cuesta tiempo de ejecución, que se puede evitar usando
    // petodos insegurosm que pueden resultar en desbordamientos.
    
    unsafe{
        println!("Se comportan un poco como modulos:");
        println!("300.0 as u8 = {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 = {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("NAN as u8 = {}", f32::NAN.to_int_unchecked::<u8>());
        
    }
}
