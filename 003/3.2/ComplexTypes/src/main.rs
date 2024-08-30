fn main() {
    let x:bool = true;
    let x = !x;
    println!("x is {}", x);
    let emoji:char = '😻';//i have no idea what would come in the terminal
    println!("i have no idea what would come in the terminal by printing this {}", emoji); // it prints 😻
    let tup:(i32,f64,char) = (10,3.0,'😻');
    println!("{}", tup.2);
    let (x,y,z) = tup;
    println!("{} {} {}", x,y,z);
    let array:[usize;5] = [1,2,3,4,5];
    println!("{}",array[2]);
    let a = [3; 5];  // 3 3 3 3 3;
    

}
