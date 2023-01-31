struct Puntos{
    x: f32,
    y: f32,
}

struct Rectangle{
    superior_izquierda: Puntos,
    inferior_derecha: Puntos,
}

fn area_rectangulo(r: &Rectangle)-> f32{
    let largo = r.inferior_derecha.x - r.superior_izquierda.x;
    let alto =  r.inferior_derecha.y - r.superior_izquierda.y;
    println!("Base: {}\nAltura: {}", largo, alto);
    largo*alto
}

fn area_cuadro(p: Puntos, l: f32)->f32{
    println!("inicia en x{} y{}", p.x, p.y);
    println!("Largo = {}", l);
   l*l
}
fn main() {
    println!("\nRectángulo: ");
    let sup_izq = Puntos {x:34.6, y:10.3};
    let inf_der = Puntos {x:199.3, y:300.4};
    let rectangulo = Rectangle{superior_izquierda: sup_izq, inferior_derecha: inf_der};
    println!("Area rectángulo: {}", area_rectangulo(&rectangulo));


    println!("\nCuadrado:");
    let lado: f32 = 32.2f32;
    let sup_izq = Puntos {x:34.6, y:10.3};
    println!("Area cuadro: {}", area_cuadro(sup_izq, lado));
    
}
