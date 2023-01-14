fn main() {
    println!("1 + 2 = {}!", 1u32 + 2);
        println!("1 + 2 = {}!", 1i32 + 2);
        println!("verdaderp y falso {}!", true && false);
        println!("verdaderp o falso {}", true || false);
        println!("no verdadero  {}", !true);

        println!("0011 & 0101 = {}!",0b0011u32 & 0b0101);
        println!("0011 | 0101 = {}!", 0b0011u32 | 0b0101);
        println!("0011 ^ 0101 = {}!", 0b0011u32 ^ 0b0101);
    println!("1u32 << 5   = {}!", 1u32 << 5);
    println!("0x80u32 >> 2   0x80 >> 2 is 0x {:x}!", 0x80u32 >> 2);

    println!("un millon es {}",1_000_000u32);
}
