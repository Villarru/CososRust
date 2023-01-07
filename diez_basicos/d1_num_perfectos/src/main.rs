use std::io;

fn pow(b: i32, exp: i32) -> i32{ 
    let mut res: i32 = b;
    for _ in 1..exp {
        res = res * b;
    }
    res
}

fn main() {
    println!("Ingresa un numero natural: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect(" error al recibir numero ");

    let num: i32 = num.trim().parse()
        .expect(" error al convertir numero ");

    let mut algo: i32 = 2;
    println!("Los numeros perfectos que se encuentran entre 1 y {} son:", &num);
    loop{
        let perf = (pow(2, &algo-1))*(pow(2,algo)-1);
        if perf > num {
            break;
        }
        
        println!("{}, ", perf );
        algo += 1;
    }
}
