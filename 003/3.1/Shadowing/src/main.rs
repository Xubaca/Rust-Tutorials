fn main() {
    let spaces = "   ";
    let spaces = spaces.len(); //se fosse mut não poderia fazer isto;
    println!("The value of spaces is : {} !", spaces);
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {} !", x);
    }
    println!("The value of x is: {} !", x);

    let y: u32 = 15;
    let y1: u32 = 16;
    println!("O resultado de 15-16 em u32 é {} !", (y + 1) - y1); // ok pelos vistos dá Overflow, por isso não dá
}
