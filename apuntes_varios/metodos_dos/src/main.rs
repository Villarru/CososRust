#[derive(Debug)]
struct Rectangulo {
    alto: u32,
    largo: u32,
}
// Uso de dos metodos.
impl Rectangulo{
    fn area(&self) -> u32{
        self.alto * self.largo
    }

    fn le_cabe(&self, algo: &Rectangulo) -> bool{
        self.alto > algo.alto && self.largo > algo.largo
    }
}

// Funciones asociadas. Self debe ser en mayuscuas, esto devuelve una variante
// de rectángulo con todos sus lados iguales.
impl Rectangulo{
    fn cuadrado(medida: u32) -> Self{
        Self{
            alto: medida,
            largo: medida,
        }
    }
}
// Los impl multiples sirven en algunos casos, pero generalmente no hay razón
// para separarlos.
fn main() {
    let rect1 = Rectangulo{
        alto: 100,
        largo: 24,
    };

    let rect2 = Rectangulo{
        alto: 91,
        largo: 234,
    };

    let rect3 = Rectangulo{
        alto: 24,
        largo: 24,
    };

    let cuadro = Rectangulo::cuadrado(34);
    dbg!(cuadro);
    println!("area = {} ", &rect1.area());
    println!("Rect1 pude contener rect2? {}", &rect1.le_cabe(&rect2));
    println!("Rect2 pude contener rect3? {}", &rect2.le_cabe(&rect3));
}
