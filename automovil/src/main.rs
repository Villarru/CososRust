// Usar enum, struct y fn para poder procesar información de automoviles.
struct Auto {color: String, motor: Transmision, techo: bool, estado: (Estado, u32),}
#[derive(PartialEq, Debug)]
enum Transmision{Manual, SemiAuto, Automatica,}
#[derive(PartialEq, Debug)]
enum Estado {Nuevo, Usado,}

fn calidad (kilometraje: u32) -> (Estado,u32){
    if kilometraje > 0 {return (Estado::Usado, kilometraje)}
    (Estado::Nuevo, kilometraje)
}

fn fabrica_auto(num:i32, color: String, motor: Transmision, techo: bool, kilometraje: u32){
    let carro = Auto{color, motor, techo, estado: calidad(kilometraje)};
    imprimir_info(&carro,num);
} 

fn imprimir_info(carro: &Auto, num: i32){
    let mut techo = String::from("");
    if carro.techo { techo = String::from("Es convertible\n ");}
    let (estado, km): &(Estado, u32) =  &carro.estado;
    
    hr();
    println!(" --- Auto {} --- \n Color: {}\n Motor: {:?}\n {}Estado {:?}\n Kilometraje: {}",
             num, &carro.color, &carro.motor, techo, estado, km);
}

fn hr(){
    println!("=============================");
}

fn main() {
    fabrica_auto(1, String::from("Verde"),Transmision::Automatica, false, 0);

    fabrica_auto(2, String::from("Negro"),Transmision::Manual, true, 0);

    fabrica_auto(3, String::from("Anaranjado"),Transmision::SemiAuto, false, 125);

    fabrica_auto(4, String::from("Morado"),Transmision::Automatica, true, 2560);
    hr();
}
