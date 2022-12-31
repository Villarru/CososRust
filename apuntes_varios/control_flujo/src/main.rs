use std::io;

fn main() {
    //expresiones if de toda la vida
    
    let mut entrada = String::new();
    const LIMITE: i8 = 9; 
    println!("introduce un numero pequeño");

    io::stdin().read_line(&mut entrada)
        .expect("error al recibir dato");

    let numero: i8 = entrada.trim().parse()
        .expect("error al convertir\n");
    
    //comprobar(numero);
    divisible(numero);
    contar(comprobar(numero, LIMITE), LIMITE);

    // se puede sar if en un let.
    let boleano: bool = true;
    let numero: i16 = if boleano { 257 } else {6};
    println!("{}", numero);

    //Ciclos: loop, while y for.
   
}

fn comprobar(num: i8, limite: i8) -> i8{
    if num > limite {
        println!("El numero {} es mayor que {}", num, limite);
        return 0;
    } else {
        println!("El numero {} es menor que {}", num, limite);
        return num;
    }
}

fn divisible(num: i8){
    if num % 5 == 0 {
        println!("El numero {} es divisible entre 5", num);
    } else if num % 4 == 0 {
        println!("El numero {} es divisible entre 4", num);
    } else if num % 3 == 0 {
        println!("El numero {} es divisible entre 3", num);
    } else if num % 2 == 0 {
        println!("El numero {} es divisible entre 2", num);
    } else {
        println!("El numero {} es chido", num);
    }

}

fn contar(num: i8, limite: i8){
    let mut contador: i32 = 1;
    println!("Fui al cerro a contar osos");
    if num == 0 {
        println!("pero no habia ninguno.");
    }else{
        loop{
            if contador != 1 {
                println!("{} osos", contador);
            } else{
                println!("{} oso", contador);
            }
            contador+=1;
            
            if contador == num.into(){
                println!("en fin, {} osos en total.",contador);
                break if num >= (limite-2) {println!("casi me muero che");};
            }
       }
   }
}






