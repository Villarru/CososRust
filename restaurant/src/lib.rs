 mod frontal {
    pub mod recepcion {
	pub fn agregar_a_lista() {
	    println!("En lista de espera...");
	}

	pub fn asignar_mesa() {
	    println!("Se le ha asignado la mesa {}.", 9);
	}
    }
 }
fn entregar_orden(){}

mod atras {
    pub struct Desayuno { // lo de acá es privado predeterminado
	pub tostada: String,
	fruta_de_temporada: String,
    }

    pub enum Aperitivos{ // lo de acá es público predeterminado
	Sopa,
	Ensalada
    }

    impl Desayuno {
	pub fn verano(tostada: &str) -> Desayuno {
	    Desayuno {
		tostada: String::from(tostada),
		fruta_de_temporada: String::from("Durazno"),
	    }
	}
    }
    
    fn arreglar_orden_equivocada(){
	cocinar_orden();
	super::entregar_orden(); //le pide al ambito superior, en esta caso crate
    }
    fn cocinar_orden(){}
}

use crate::atras::Aperitivos; // este solo crea el atajo para un ámbito
// se puede combinar con super para que haga referencia a alguna cosa del
// modulo padre.

pub fn comer_en_restaurante() {
    crate::frontal::recepcion::agregar_a_lista();
    frontal::recepcion::agregar_a_lista();

    let mut comida = atras::Desayuno::verano("Integral");
    comida.tostada = String::from("trigo");
    println!("Quiero una tostada de {}", comida.tostada);
    let orden1 = Aperitivos::Sopa;
    let orden2 = Aperitivos::Ensalada;
}
