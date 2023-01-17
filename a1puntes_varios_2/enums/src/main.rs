enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{ x:i64, y:i64},
}

fn inspect(event: WebEvent){
    match event{
        WebEvent::PageLoad => println!("Página cargada "),
        WebEvent::PageUnload => println!("Página no cargada "),
        // Dice que aca se destructuró.
        WebEvent::KeyPress(c) => println!("Has presionado {}", c),
        WebEvent::Paste(s) => println!("Has pegado: \"{}\" ", s),
        // Esto tambien se destructuró.
        WebEvent::Click{ x, y } => {
            println!(" cliked at x={} y={}", x, y);
        },
    }
}

// Ahí te va el alias pa
type We = WebEvent;
// Estas maniobras se ven mas en los impl con el alias Self.

fn main() {
    let pressed = We::KeyPress('w');
    let pasted = We::Paste("String desde &str gracias al metodo to owned".to_owned());
    let click = We::Click { x: 120, y: 380};
    let load = We::PageLoad;
    let unload = We::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    
    println!("Hello, causa!");
}
