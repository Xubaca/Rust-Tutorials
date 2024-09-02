fn main() {
    let passed_ref = 10;
    let arg :i32 = {returned_value(&passed_ref)+1};
    println!("The final answer is:\t {}!;", arg);
    println!("The value of the passed reference out of the scope is:\t {}!;", passed_ref);//should be 10
    println!("The value of the returned value is:\t {}!;", early_return());
    println!("The value of the returned tuple is:\t {:?}!;", varied_returned());
}

fn returned_value(x: &i32) -> i32 {//pass by reference
    *x*2
}

fn early_return() -> i32 {//pass by reference
    let x = 10;
    let y = 20;
    return x+y;

    10
}

fn varied_returned() -> (i32 , char) {
    (32,'a')
}