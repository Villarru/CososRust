fn main() {
    //variable bindins, parecido a definir variables pero con distinta filosofia.
    let _CINCO = 5;
    //patterns chidos
    let (x, y, z) = (3, 4, 5);
    //Rust es de tipado fuerte pero puede inferir los tipos (como los dos
    // anteriores.

    //definir de forma explicita:
    let w: i32 = 5;
    //aca dice bind la cosa W de tipo entero de 32bis de valor 5.

    let num_8bits: i8 = 126;
    let num_8bitsU: u8 = 254;

    let num_16bits: i16 = 32766;
    let num_16bitsU: u16 = 65533;
    
    let num_32bits: i32 = 2147483644;
    let num_32bitsU: u32 = 4294967288;

    let num_65bits: i64 = 9222372123123123123;
    let num_64bitsU: u64 = 18444777000777555615;

    //la U es de unsigned, osea puros valores positivos, por eso los valores
    // pueden ser superiores. por default es i32

    //por defecto todas las declaraciones anteriores son inmutables. para poder
    // cambiar sus valores se usa mut.

    let mut num_mutable: u8 = 20;
    num_mutable = 30;

    //en rust los bindinds siempre deben tener un valor inicial.
    //tambien hay scope en rust, puedes llamar bindings en un scope exterior al
    // bloque desde el interior pero no al reves.

    //el shadowin parece tratarse de una especie de redefinicion de los
    // bindings.
    println!("{}",w);
    let w = "coso"; //esto antes era un i32 de valor 5.
    println!("{}",w);
}
