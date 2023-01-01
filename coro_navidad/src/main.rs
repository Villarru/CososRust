fn main() {
    println!("========================================");
    println!("===Letra de \"A doce días de navidad\" ===");
    println!("========================================");
    const DOCE_DIAS: usize = 12;
    
    let linea_cero = "Mi amor me dio";
    let doce_dias = ["primer", "segundo", "tercer", "cuarto",
                     "quinto", "sexto", "séptimo", "octavo",
                     "noveno", "décimo", "décimo primer", "duodécimo"];
    let doce_lineas = ["Y un perdiz en un peral",
                 "Dos tórtolas",
                 "Tres gallinas francesas",
                 "Cuatro pájaros llamadores",
                 "Cinco anillos de oro",
                 "Seis gansos",
                 "Siete cisnes nadando",
                 "Ocho criadas ordeñando",
                 "Nueve damas bailando",
                 "Diez señores saltando",
                 "Once galleros tocando",
                 "Doce tamborileros tamboreando",];

    let mut contador: usize = 0;
    while contador < (DOCE_DIAS){
        println!("\n[{}]\nEl {} día de navidad\n{}", (contador+1), doce_dias[contador], linea_cero);

        if contador != 0 {
            let mut x: usize = contador; 
            loop {
                println!("{}",doce_lineas[x]);
                if x!=0 {
                    x-=1;
                }else{
                    break;
                }
            }
        }else{
            println!("Un perdiz en un peral");
        }
        
        contador+=1;
    }
}
