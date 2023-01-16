// Hay 3 tipos. tuple structs, structs clasic, unit Struct, ahora dejaré todo
// en inglés.
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// notese que para estos dos se necesita ;
struct Unit;
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Patrick");
    let age = 39;
    let patrick = Person{name, age};

    println!("persona:{:?} ", patrick );

    let point: Point = Point {x: 23.1, y: 1.3};

    println!("Coordenadas : x{}, y{}", point.x, point.y);

    // aca se usa como valor de "y" con el ultimo valor de point
    let bottom_right = Point {x: 3.4, ..point};

    println!("Segundo punto: {} {}", bottom_right.x, bottom_right.y);

    // Destructurar el punto con vinculo let.
    let Point {x: left_edge, y: top_edge} = point;

    let _rectagle = Rectangle{
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right, // el de la linea 35
    };

    let _unit= Unit;

    let pair = Pair(1, 0.1);

    println!("par: {:?} y {:?}", pair.0, pair.1);
    
}
