#![allow(unused_variables)]

#[allow(dead_code)]
fn square(x: i32) -> i32 {
    let temp = 42;
    x * x
}

// #[allow(unused_variables)]
pub fn exec() {
    println!("COMPILER DIRECTIVES");
    // #[allow(unused_variables)]
    let i = 10;
    let s = String::from("Hi there");

    println!("-----\n");
}