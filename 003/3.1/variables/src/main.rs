const SECS_PER_HOUR: u32 = 60 * 60;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    SECS_PER_HOUR;
    
    //changing the type with shadowing

    let spaces: &str = "    ";
    let spaces: usize = spaces.len();
    println!("{}", spaces);
}
