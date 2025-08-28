pub fn exec() {
    println!("VARIABLES AND CONSTANTS");
    let x: i16 = 10;
    println!("x is: {x}");

    // Mutability
    // let y: i32 = 5;
    // y = 10;
    let mut _y: i32 = 5;
    _y = 10;

    // Scope
    {
        let _z: i32 = 50;
    }
    // let s = _z;

    // Shadowing
    let t: i32 = 10;
    let t: i32 = t + 10;
    println!("t is: {t}");

    let u: i32 = 3;
    println!("u is: {u}");
    let u: f64 = 3.0;
    println!("u (shadowed) is: {u}");

    let v: i32 = 30;
    {
        let v: i32 = 40;
        println!("Inner v is: {v}");
    }
    println!("v is: {v}");

    // Constants
    const MAX_VALUE: u32 = 100;
    println!("MAX_VALUE is: {MAX_VALUE}");

    println!("-----\n");
}