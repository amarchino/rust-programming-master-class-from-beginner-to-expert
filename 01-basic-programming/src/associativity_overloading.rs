pub fn exec() {
    println!("ASSOCIATIVITY AND OPERATOR OVERLOAD");

    // Associativity
    let x = 8 / 4 / 2;
    println!("x is {x}");

    // Explicit boolean in conditionals
    let x = 0;
    // if x {} // Error
    if x != 0 {}

    // Operator overloding
    let a = 10 + 20;
    let b = String::from("1") + " 2";
    println!("a: {a}, b: {b}");

    println!("-----\n");
}