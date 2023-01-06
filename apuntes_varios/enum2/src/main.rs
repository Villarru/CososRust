// Otra forma de usar los enum es pidiendo directamente la data

enum DirIP{
    V4(u8,u8,u8,u8), // porque IPv4 son 4 grupos de 0 a 255
    V6(String),
}

// Tambien puedes poner estructuras enum
struct Quitar;
struct Mover { x: i32, y: i32,}
struct Escribir(String);
struct Color(u8, u8, u8);

#[derive(Debug)]
enum Mensaje {
    Quitar,
    Mover {},
    Escribir(String),
    Color(u8, u8, u8),
}

// Y se le pueden definir metodos a los enum.
impl Mensaje {
    fn llamar(&self){
        dbg!(&self);
    }
}


// Otro uso para enum es el manejo de null (que no son nulls)
// enum Option1<T>{
   // None,
   // Some(T),
//}

fn main() {
    let local = DirIP::V4(127,0,0,1);
    let loopback = DirIP::V6(String::from("::1"));
    
    let m = Mensaje::Escribir(String::from("Holassasas"));
    m.llamar();

    // Ultima nota de enum
    let algun_numero = Some(99);
    let algun_char = Some('S');

    let num_ausente: Option<i32> = None;
    // el primer valor es Option<i32>, el segundo es Option<char> y el ultimo esta
    // confuso. Es de tipo i32 pero no tiene algo guardado, es parecido a null
    // pero tiene una maña, es un tipo de dato que no sirve para trabajarse con
    // otros datos que no sean de su tipo. Por eso el enum.

    // Esa caracteristica te obliga atransformar Option<T> en T para poder
    // trabajarse, así te evitas un problema común con los nulos que es asumir
    // que un valor ya no es nulo cuando sí lo es.

    
    
    
}
