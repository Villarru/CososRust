use std::io;

fn main() {
    //consantes:
    const DOS_HORAS_A_SEGUNDOS: u32 = 60 * 60 * 2;
    println!("Constante dos horas a segundos: {}", DOS_HORAS_A_SEGUNDOS);

    //el shadowin es como reescribir un binding desde 0 pero este puede volver
    // a su estado original bajo ciertas circunstancias.

    let coso: i16 = 34;
    println!("coso binding i16 con valor = {}", coso);
    loop{
        let coso = "ahora coso es string";
        println!("{}", coso);
        break;
    }
     println!("ahora coso vuelve a ser {}", coso);

    //es distinto a una variable mut, esta reemplaza el original con un dato
    // nuevo del mismo tipo. por otro aldo el shadowing sale mal si no se usa
    // "let"

    //no lo habia anotado antes pero en valores numericos tambien hay i128 y
    // u128
    let _numerote: i128 = 123412341234;

    //y esta el isize y usize que depende de la arquitectura del ordenador que
    // se esta ejecutando. por ejemplo 64bits o 32bits.

    //los numeros se pueden declarar de diferentes maneras:

    let decimal: i64 = 1_346_345_654;
    let hexadecimal: i32 = 0xDF;
    let octal: i32 = 0o77;
    let binario: i16 = 0b0111_1000;
    let byte_u8solo: u8 = b'A';


    println!("\n1_346_345_654 = {}\n0xDF = {} \n0o77 = {} \n0b0111_1000 = {} \nbyte b'A' = {}",
             decimal, hexadecimal, octal, binario, byte_u8solo);

    //punto flotante f32 y f64
    let _flotante: f32 = 1998.25;

    //el f64 es doble presición.

    //operadores numericos de toda la vida
    let suma = 5 + 5;
    let resta = suma - 2;
    let multipli = resta * suma;
    //divisiones
    let division_completa = 9.5 / 3.4;
    let division_redondeada = 45 / 11;

    let modulo = 34 % 7;
    // al parecer las operaciones deben ser entre el mismo tipo de datos, si
    // tienes bind f32 y le quieres poner el resultado de un i32 mas un f32
    // dara error. 
    println!("\nOtros asuntos (ver codigo): \n   > suma  = {} \n   > resta  = {}
   > multipli  = {} \n   > div completa  = {}\n   > div redondeada  = {}\n   > modulo  + {}",
             suma, resta, multipli, division_completa, division_redondeada, modulo
    );

    
    //BOLEANOS
    let _t = true;
    let _f: bool = false;
    //sin y con notación explícita

    //caracter

    let _c = 'u';
    let _o: char = 'U';
    //tambien los emojis valen como char, noto que se usan comillas sencillas.
    //RUST tambien jala con acentos, caracteres chinos, japonenes y coreanos, y
    // 'zero-width spaces'. Ademas del ASCII. U+0000 hasta u+D7FF y U+e000 a
    // U+10FFFF.

    //Todo lo anterior fueron Scalar Types. Ahora tocan tipos compuestos.

    //tuplas

    let tupla: (i32, f32, u8) = (346, 7.7, 1);

    //ojo que esto es diferente al mutiple bind
    let (_coso1, _coso2) = (14, 3);

    //pero se puede usar para sacar datos de una tupla

    let (a, b, c) = tupla;

    println!("\nImpresion de datos de tupla partida: [{}] [{}] [{}]",a,b,c);

    //tambien se pueden sacar con un punto.

    let primero = tupla.0;
    let tercero = tupla.2;

        println!("\nImpresion de datos de tupla accediendo con punto: [{}] [{}] [{}]", primero, tupla.1, tercero);


    //ARRAY
    let _arreglo = [5, 6, 7, 8, 9];
    //o de forma explicita
    let arr: [i32; 5] = [3, 4, 5, 6, 7];

    //tambien se puede llenar con el mismo dato:
    let _arreglo = [7; 3]; //equivale a let arreglo: [i32; 3] = [7, 7, 7];
    
    //a estos se accede con nombre_arreglo[valor numerico]:
    println!("\nArreglo valor 1: {} \ny arreglo 777 valor 2: {}", arr[0], _arreglo[2]);

    let mut valor = String::new();
    println!("El arr tiene [3,4,5,6,7] elige un numero: ");

    io::stdin().read_line(&mut valor)
        .expect("problema al leer dato");

    let valor: usize = valor.trim().parse()
        .expect("Error al convertir a numero de 16bits");

    //otra forma de imprimir.

    let resultado_consulta = arr[valor-1];

    println!("El valor de arr que elegiste {valor} es {resultado_consulta}");
    
}
