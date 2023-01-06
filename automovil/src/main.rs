// Usar enum, struct y fn para poder procesar información de tres automóviles nuevos.

struct Auto {
    color: String,
    transmision: Transmision,
    convertible: bool,
    kilometraje: u32,
}

#[derive(PartialEq, Debug)]
enum Transmision{
    Manual,
    SemiAutom,
    Automatica,
}

fn fabrica_auto(color: String, transmision: Transmision, convertible: bool) -> Auto{
    let auto = Auto{color, transmision, convertible, kilometraje: 0, };
    auto
}
fn hr(){
    println!("=============================");
}

fn imprimir_auto(carro: &Auto, num: i32){
    hr();
    println!(" --- Auto {} --- \n Color: {}\n Trasmision: {:?}\n Convertible: {}\n Kilometraje: {}",
             num, carro.color, carro.transmision, carro.convertible, carro.kilometraje);
}

fn main() {
    let mut carro: Auto = fabrica_auto(String::from("Verde"),Transmision::Automatica, false);
    imprimir_auto(&carro,1);
    
    carro = fabrica_auto(String::from("Negro"),Transmision::Manual, true);
    imprimir_auto(&carro,2);
    
    carro = fabrica_auto(String::from("Anaranjado"),Transmision::SemiAutom, false);
    imprimir_auto(&carro,3);

    hr();
}
