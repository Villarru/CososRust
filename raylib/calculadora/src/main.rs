//extern crate raylib;
use raylib::prelude::*;

struct Btn {
    alto: i32,
    ancho: i32,
}

fn main() {
    let (ancho, alto) = (600, 450);
    let (mut rl, thread) = raylib::init()
	.size(ancho, alto)
	.title("Calculadora")
	.build();
    let boton = Btn{ alto:35, ancho:160 };
    let letra_size = 16;
    let mut mensaje = "Hlow modafaka prestas siuu";
    
    while !rl.window_should_close() {
	let mut d = rl.begin_drawing(&thread);
	d.clear_background(Color::WHITE);
	d.draw_text(mensaje, ( ancho / 2 as i32 ) as i32 - ((mensaje.len()*(letra_size)+1)/4) as i32, 12, 18, Color::BLACK);
	d.draw_rectangle((ancho/2 as i32) - (boton.ancho/ 2 as i32), 40, boton.ancho, boton.alto, Color::GRAY);

    }
}
