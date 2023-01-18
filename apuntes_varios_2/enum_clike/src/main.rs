#![allow(dead_code)]

// implícito
enum Number {
    Zero,
    EsteEsElNumeroUno,
    Two,
}
// explícito
enum Color{
    Red = 0xff0101,
    Green = 0x01ff01,
    Blue = 0x0101ff,
}

fn main() {
    // Casteando como enteros:
    // Implicito iniciando en 0
    println!("Cero es {}", Number::Zero as i32);
    println!("Uno es {}", Number::EsteEsElNumeroUno as i32);

    //Explicito en 3 grupos hexadecimales
    println!("Las rosas son #{:06x}", Color::Red as i32);
    println!("Las violetas son #{:06x}", Color::Blue as i32);
}
