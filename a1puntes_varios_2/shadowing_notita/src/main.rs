fn main() {
    let cosa = 12;
    let mut numero = 1;
    let string = String::from("Ansi es");
    println!("Coso = {} y num = {}", cosa, numero);
    {
        
        println!("scope (");
        // si haces asi no pasa nada, se hace referencia y ya.
        println!("{}",string);
        let string = "A ver si cambia"; // se hizo modifiacion local
        println!("{}",string);
        
        let cosa = "Cosaaa";
        numero = 34;
        println!("Modificar coso con let y numero sin let:\nCoso = {} y num = {}", cosa, numero);
        let numero = 99;
        println!("let numero = 99: \nCoso {} y num {}\n) fin de scope\n", cosa, numero);
    }
    println!("fuera de scope: Coso {} y num {}", cosa, numero);
    let cosa = 23;
    let numero = 61;
    println!("ultima modificación: Coso {} y num {}", cosa, numero);
    println!("{}",string);
    
}
