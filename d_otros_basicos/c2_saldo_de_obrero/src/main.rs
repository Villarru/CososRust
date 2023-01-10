use std::io;
fn main() {
    println!("Obrero que recibe $50 por hora, tiene compensacón de 2%, descuento de 1.5% para IMSS y debe 1.92% de ISR. ");
    println!("Introduce cantidad de horas que obrero trabajó en el mes (0hrs a 220hrs): ");
    let mut dato = String::new();
    io::stdin().read_line(&mut dato).expect(" error al recibir horas trabajadas ");

    let dato: f32 = dato.trim().parse().expect(" error al convertir entrada a horas ");

    let pago_neto: f32 = (dato*50.0)*1.02;
    let descuento_imss: f32 = pago_neto*0.015;
    let descuento_isr: f32 = (pago_neto-descuento_imss)*0.0192;
        println!("{} horas trabajadas.\nSalario más compensacón de 2%: ${}\nIMSS: - {}\nISR: -{}\nSalario final: ${}"
                 , &dato , &pago_neto, &descuento_imss, &descuento_isr, pago_neto-(descuento_imss+descuento_isr));
}
