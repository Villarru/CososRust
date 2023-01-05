#[derive(Debug)] //explicaci[on en linea 34
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

fn main() {
    let ancho1 = 160;
    let alto1 = 60;
    
    println!("==================================");
    println!("Ancho {}, alto {}\nEl area es = {}", ancho1, alto1, area(ancho1,alto1));
    println!("==================================");
    
    //como el bisnes del area es indistinto el orden de los factores se puede
    // refactorizar todo con tuplas.
    let rectangulo1 = (60, 90);

    println!("El area es {} (por tupla)", area_tupla(rectangulo1));
    println!("==================================");
    //aunque para hacer el cálculo no es importante el orden, puede que si lo
    // sea para insertar los datos, habría que diferenciar. por ejemplo si
    // queremos dibujar un rectangulo en pantalla.

    let rectangulo2 = Rectangulo {
        ancho: 90,
        alto: 60,
    };

    println!("El area es {} (por estructura)", area_estructura(&rectangulo2));
    println!("==================================");

    //las estructuras no se pueden imprimir con println de forma nomral, pero
    // se pueden imprimir con funcionalidades de debug usando un :? dentro del {}

    println!("Detalles de rectangulo2: \n{:?} \n\n(impreso con #[deribe(Debug)] y :?)",rectangulo2);
    println!("==================================");

    println!("Detalles de rectangulo2: \n{:#?} \n\n(impreso con #[deribe(Debug)] y :#?)",rectangulo2);
    println!("==================================");

    //Y hablando de imprimir cosas raras. dbg!(); tambien ayuda a ver esa info,
    // pero solo se puede usar con la compilación en modo debug.

    dbg!(&rectangulo2);
    //imprime parecido a lo que ahora está en la linea 25.
}

fn area(x: i32, y: i32) -> i32{
    x*y
}

fn area_tupla(dimensiones: (i32, i32)) ->i32{
    dimensiones.0 * dimensiones.1
}

fn area_estructura(r: &Rectangulo)-> u32{
    r.ancho * r.alto
}
