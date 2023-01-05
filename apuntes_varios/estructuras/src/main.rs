fn main() {
    //las estructuras se ven un poco como los json.

    //no necesariamente debe tener el mismo orden
    let mut usuario = User {
        conteo_accesos: 1,
        nombre: String::from("Usuario67"),
        activo: true,
        correo: String::from("usuario67i@gmail.com"),
    };

    usuario.activo = false;

    //se puede crear un usuario nuevo a partir de 
    let usuario2 = User {
        conteo_accesos: usuario.conteo_accesos+1,
        ..usuario  //Esto equivale a:
        //nombre: usuario.nombre,
        //ctivo: usuario.activo,
        //correo: usuario.correo,
    };
    // A partir de aca ya no se pueden usar los campos de correo y nombre del
    // User usuario porque los string ahora le pertenecen a usuario2
    
    let negru = Color(0,0,0);
    let origen = Punto(0,0,0);
    
    println!("Usuario: {} \nCorreo: {} \nActivo: {}\nAccesos: {}\n=======================",
             usuario2.nombre, usuario2.correo, usuario2.activo, usuario2.conteo_accesos);

    let (nombre3, correo3) = (String::from("SenA_Door1"), String::from("Senadorino@gmail.com"));

    let usuario3 = constructor_usuario(nombre3, correo3);

    println!("Usuario: {} \nCorreo: {} \nActivo: {}\nAccesos: {}\n=======================",
             usuario3.nombre, usuario3.correo, usuario3.activo, usuario3.conteo_accesos);

    
    println!("negro es rgb({}, {}, {})", negru.0, negru.1, negru.2);
    println!("La coordenada es en los puntos: x {}, y{}, z{})", origen.0, origen.1, origen.2);
}

struct User{
    activo: bool,
    nombre: String,
    correo: String,
    conteo_accesos: u64,
}

//estructuras tipo tuplas, sin nombres ni tanto show.
struct Color(i32, i32, i32);
struct Punto(i32, i32, i32);


fn constructor_usuario(nombre: String, correo: String) -> User{
    User{
        activo: true,
        conteo_accesos: 1,
        correo, //esto es una form acortada de correo: correo, rust le sabe.
        nombre,  
    }
}


