fn print_number(x: i32) {
    // x = x + 2; // Error
    println!("x is {x}");
}
fn print_number2(mut x: i32) {
    x = x + 2;
    println!("x is {x}");
}
fn print_number3(x: i32) {
    let mut x = x;
    x = x + 2;
    println!("x is {x}");
}
fn print_number4(x: i32) -> i32 {
    let mut x = x;
    x = x + 2;
    println!("x is {x}");
    x
}

pub fn exec() {
    println!("MUTABILITY IN FUNCTION PARAMETERS");
    #[allow(unused_mut)]
    let mut num = 10;
    print_number(num);
    print_number2(num);
    print_number3(num);
    #[allow(unused_variables)]
    let y = print_number4(num);
    // y += 2; // Error

    println!("-----\n");
}