// Sobre el aliasing tomé notas en la carpeta enum. type se usa para darle un
// nuevo nombre a un tipo existente. debe usar UperCamelCase, con excepcion de
// los tipos primitivos.

type NanoSegundo = u64;
type Pulgada = u64;
type U64 = u64;

fn main() {
    // se usa un sufijo
    let a = 12u8;
    let b = 34u16;
    let c = 567i32;
    // el tipo depende de su uso
    let x = 1;
    let y = 0.1;
    // uso de aliashing
    let nano_seg: NanoSegundo = 57 as U64;
    let pulgada: Pulgada = 234 as U64;
    
    println!("espacio de 'a' en bytes es: {}", std::mem::size_of_val(&a)); // 1
    println!("espacio de 'b' en bytes es: {}", std::mem::size_of_val(&b)); // 2
    println!("espacio de 'c' en bytes es: {}", std::mem::size_of_val(&c)); // 4
    println!("espacio de 'x' en bytes es: {}", std::mem::size_of_val(&x)); // 4
    println!("espacio de 'y' en bytes es: {}", std::mem::size_of_val(&y)); // 8
    // este ultimo lo tomó como f64

    println!("\nAliashing: ");
    println!("{} nanosegundos + {} pulgadas = {}", nano_seg, pulgada, nano_seg+pulgada);

    println!("\nVector: ");
    // inferencia
    let mut vec = Vec::new(); // arreglo agrandable.
    vec.push(a);
    // no especificamos el tipo de datos que tendrá el vector, pero el
    // compilador sabe, por el tado que agregamos, que seraun Vec<u8>.
    println!("{:?}", vec);
    // Inferencia, hommie.
}
