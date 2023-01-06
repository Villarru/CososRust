// El enum lista TIPOS de datos similares. como los dos tipos de direciones IP,
// o un set de figuras como rectángulo, cuadrado, triángulo, etc. Cuando solo
// pueda ser un tipo de una lista de opciones.

enum dir_ip_tipo {
    V4,
    V6,
}
// Ahora podemos usar el tipo de dato dir ip tipo como entrada, o especificar
// cual tipo de dato necesitamos.

struct direccion_ip {
    tipo: dir_ip_tipo,
    direccion: String,
}


fn main() {
    let cuatro = dir_ip_tipo::V4;
    let seis = dir_ip_tipo::V6;

    let local = direccion_ip {
        tipo = dir_ip_tipo::V4,
        direccion = String::from("127.0.0.1"),
    }

    let loopback = direccion_ip {
        tipo = dir_ip_tipo::V6,
        direccion = String::from("::1"),
    }
    
}
