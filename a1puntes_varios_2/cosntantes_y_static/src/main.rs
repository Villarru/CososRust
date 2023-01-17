// Es como variable global
static LANG: &str = "RUST";
// inmutable e inshadowineable.
const UMBRAL: i32 = 10;

fn is_big(x: i32)-> bool{
    x > UMBRAL
}

fn main() {
    let num = 45;

    println!("Programado en {}", LANG);
    println!("El umbral es: {}", UMBRAL);
    // Truquito con if dentro del print
    println!("{} es {}", num, if is_big(num){ "grande"} else { "pequeño" });

    // Este no se puede cambiar:
    // UMBRAL = 1;
    // Y esto da error
    // LANG = "eit";
}
