fn main() {
    println!("Hola");

    let mut texto = String::from("Ansí es");
    println!("Está \"texto\" en casa? {}", texto);

    texto = tomar_valor_prestado(texto);

    robar_propiedad(texto);
    
    //println!("hora del show {}", texto);
    //Curiosamente el error de compilación es bastante claro
    //cuando haces cargo run, bastante detalle sobre lo que pasa acá..

    //En fin, chequea esto pa:
    let cadena1 = String::from("Ve el trucazo");

    let (cadena2, largo) = calcular_largo(cadena1);

    println!("La longitud de \"{}\" es \"{}!\", ya todos se la saben.", cadena2, largo);

    //antes devolvimos dos valores, pero si pido prestado el cadena 2 no
    // tendría que devolverlo.
    println!("\nEste texto usó referencias para calcular que {} es la longitud de \"{}\"", calcular_largo_2(&cadena2), cadena2);
    //así se puede pasar cadena2 sin que pierda el ownership y sin tener que
    // devolverlo. Aun me queda ver el tema de borrowing.
    
}

fn robar_propiedad(texto_robado: String){
    println!("Ahora texto string no deberia poder usarse fuera de este ambito? {}", texto_robado);
}
fn tomar_valor_prestado(valor: String) -> String{
    println!("Prestas y regreso, {}", valor);
    valor
}

fn calcular_largo(a: String) -> (String, usize) {
    let longitud = a.len();
    //debo admitir que subestimé el poder de las tuplas.
    //y espera a ver el uso de &.
    (a, longitud)
}

fn calcular_largo_2(s: &String) -> usize {
     s.len()
}
