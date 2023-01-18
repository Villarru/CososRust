use crate::List::*;

enum List{
    // Tupla con puntero y valor.
    Cons(u32, Box<List>),
    // El fin de la lista enlazada.
    Nil,
}

// petodos para trabajar la lista con el enum.
impl List{
    // Funcion new que retorna un Nil de tipo List.
    fn new() -> List {
        Nil
    }
    // Este te crea un nodo, le pasas el elemento y un numero.
    fn prepend(self, elem:u32) -> List{
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32{
        // Algo pasó luego del 2018 que self debe ser referenciado y pasar por
        // un match de esta forma:
        match *self {
            // Como self es referencia, no puedes tomar el ownership de tail
            // por eso se usa ref.
            Cons(_, ref tail) => 1 + tail.len(),
            // En caso de que la lista esté vacía.
            Nil => 0
        }
    }

    // To string, textificación
    fn stringify(&self)-> String{
        match *self{
            Cons(head, ref tail) =>{
                // Format es como print pero regresa heap allocated string. 
                format!("{}, {}", head, tail.stringify())
                // recursividad jefe.
            }
            Nil =>{
                format!("Nil.")
            }
        }
    }
    
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(3);
    list = list.prepend(6);
    list = list.prepend(8);
    list = list.prepend(9);
    list = list.prepend(33);
    
    println!("La lista enlazada mide: {}", list.len());
    println!("{}", list.stringify());
}
