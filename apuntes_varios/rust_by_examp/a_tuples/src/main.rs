// Las tuplas pueden regresarse desde unma funcion
use std::fmt;

fn invertir(par: (f32, f32))-> (f32, f32){
    let (int_param, bool_param) = par; // truquito con let
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matriz(f32, f32, f32, f32);

impl fmt::Display for Matriz{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: Matriz)-> Matriz{
    let tupla1 = invertir((m.2, m.3));
    let tupla2 = invertir((m.0, m.1));
    let x = Matriz(tupla1.0, tupla1.1, tupla2.0, tupla2.1);
    x
}

fn main() {
    // Una tupla con diferentes tipos
    let super_tupla = (1u8, 2u16, 3u32, 4u64,
                       -1i8, -2i16, -3i32, -4i64,
                       0.1f32, 0.2f64, 'a', true);
    //nomas falto isize y usize para tener todos los tipos de numeros

    println!("Gran tupla primer valor: {}", super_tupla.0);
    println!("Gran tupla segundo valor: {}", super_tupla.1);

    let tupla_de_tuplas = (true,(999i64, ('a', 'b'), false), (1u8, 3.4f32),(5u64, -6i8));

    // Estas se pueden imrpimir directamente cuando tienen menos de 12 elementos
    println!("Tupla inception: \n {:?}", tupla_de_tuplas);

    let par = (6.0, 34.0);
    println!("el par es: {:?}", par);

    println!("el par en sentido opuesto: {:?}", invertir(par));

    println!("Se requiere de una coma (13u8,) para indicar que es una tupla: {:?}",(13u8,));
    println!("De otro modo (13u8) sería un simple entero: {:?}",(13u8));


    // las tuplas se pueden destuir para hacer vinculaciones.
    let tupla=(1, "Hola", 4.5, true);
    
    let (a,b,c,d)=tupla;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    let matriz = Matriz(1.1, 1.2, 2.1, 2.2);
    // Imprimiendo con display:
    println!("Display: \n{}", matriz);
    println!("Transpose: \n{}", transpose(matriz)); 
    
}

