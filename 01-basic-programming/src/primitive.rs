pub fn exec() {
    println!("PRIMITIVE DATA TYPES");
    // Unsigned integers
    let _unsigned_num: u8 = 5; // u16, u32, u64, u128
    // Signed integers
    let _signed_num: i8 = 5; // i16, i32, i64, i128

    // Floating point numbers
    let _float_num: f32 = 5.0; // f64

    // Platform specific integers
    let _arch_1: usize = 5;
    let _arch_2: isize = 5;

    // Characters
    let _char: char = 'a';

    // Boolean
    let _b: bool = true;

    // Type aliasing
    type Age = u8;
    let _peter_age: Age = 42;

    // Type conversion
    let a: i32 = 10;
    let _b = a as f64;

    println!("-----\n");
}