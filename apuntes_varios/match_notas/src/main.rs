use std::option::Option;


// Las monedas de 10 pesos con errores de impresion tienen diferentes valores
// Tambien las monedas conmemorativas
#[derive(Debug)]
enum Edicion{
    Normal,
    BatallaPuebla,
    NuevosPesos,
}

#[derive(Debug)]
enum Moneda{
    Uno,
    Dos,
    Cinco,
    Diez(Edicion),
    Veinte,
}

fn valor_contenido(moneda: &Moneda) -> u8 {
     // Nota imporante: match debe compara TODAS las opciones del
     // enum, sino da error.
    match moneda{//pattern &moneda: Dos' not covered.
        
        Moneda::Uno => { 1 },
        //Moneda::Dos => 2,
        Moneda::Cinco => 5,
        Moneda::Diez(dif)=> {
            println!("Tipo: {:?}", dif);
            10},
        Moneda::Veinte => 20,
    }
}

fn suma_uno(x: Option<u8>) -> Option<u8>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let mut dinero: Moneda = Moneda::Cinco;
    println!("Valor de moneda de {:?} es {}", &dinero, valor_contenido(&dinero) );
    dinero = Moneda::Diez(Edicion::Normal);
    println!("Valor de {:?} es {}", &dinero, valor_contenido(&dinero) );
    dinero = Moneda::Diez(Edicion::BatallaPuebla);
    println!("Valor de {:?} es {}", &dinero, valor_contenido(&dinero) );
    dinero = Moneda::Dos;
    println!("Valor de {:?} es {}", &dinero, valor_contenido(&dinero) );
    dinero = Moneda::Uno;
    println!("Valor de {:?} es {}", &dinero, valor_contenido(&dinero) );
    dinero = Moneda::Veinte;
    println!("Valor de {:?} es {}", &dinero, valor_contenido(&dinero) );
    let dineroTotal = suma_uno(Option::Some(valor_contenido(&dinero)));
    println!("Ahora tienes {:?}", dineroTotal);
}
