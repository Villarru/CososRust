// Arreglos y slices, los arreglos son los de toda la vida y los slice son
// pedazos de datos en cadena, y siempre son referencias a algo.

use std::mem;

fn analiza(slice: &[i32]){
    println!("=============================");
    println!("Tamaño en bytes: {}", mem::size_of_val(&slice));
    println!{"Primer elemento: {}", slice[0]};
    println!("Cantidad de elementos: {}", slice.len());
    println!("Ultimo dato: {}", slice[slice.len()-1]);
}
fn main() {
    // areglo con valor conocido
    let equis: [i32;5] = [10, 20, 30, 40, 50];
    // aca son 300 veces el numero 4
    let yees: [i32;300] = [4;300];

    // aca se manda como slice completo
    analiza(&equis);
    analiza(&yees);

    // aca como slice de una sección
    analiza(&equis[1 .. 3]);
    analiza(&yees[4 .. 34]);

    // se puede usar ".get" con los arreglos y slices para impedir aceder a
    // valores mas allá del index

    for i in 0..equis.len()+1 {
        match equis.get(i){
            Some(xval) => println!("{:?}: {}", i, xval),
            None => println!("Te pasaste, no hay valor {} en ese arreglo", i),
        }
    }

    // Tambíen pueden estar vacíos
    let arreglo_con_nada: [u32; 0] = [];
    assert_eq!(&arreglo_con_nada, &[]);}
