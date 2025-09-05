pub fn exec() {
    println!("OPERATORS");
    // Arithmetic operators
    // +, -, *, /, %
    // Note: ^ is not exponential
    // pow() method is used for exponential
    println!("Remainder after dividing 17 by 5: {}", 17 % 5);

    // Comparison operators
    // Equality: == (Not to be confused with assignment, i.e. =)
    // Inequality: !=
    // Relational: >, < >=, <=
    let a = 10;
    let b = 20;
    println!(
        "a == b: {}, a != b: {}, a < b: {}, a > b: {}, a <= b: {}, a >= b: {}",
        a == b, a != b, a < b, a > b, a <= b, a >= b
    );

    // Logica operators
    // And: &&
    // Or: ||
    // Not: !
    if (a > 5) && (b < 25) {
        println!("Conditions satisfied")
    } else {
        println!("Conditions not satisfied")
    }

    // Assignment operators
    // Add and assign: +=
    // Subtract and assign: -=
    // Multiply and assign: *=
    // Divide and assign: /=
    // Remainder and assign: %=
    let mut x = 5;
    x += 5;
    x -= 5;
    x *= 5;
    x /= 5;
    x %= 5;
    println!("x is {x}");

    // Bitwise operators (only on integer and unsigned)
    // And: &
    // Or: |
    // Xor: ^
    // Not: !
    // Left shift: <<
    // Right shift: >>

    let x: u8 = 4;
    let y: u8 = 7;
    println!("{}", x & y); // => 0000_0100 & 0000_0111 = 0000_0100 = 4
    println!("{}", x | y); // => 0000_0100 | 0000_0111 = 0000_0111 = 7
    println!("{}", x ^ y); // => 0000_0100 ^ 0000_0111 = 0000_0011 = 3
    println!("{}", !x); // => !0000_0100 = 1111_1011 = 251
    println!("{}", x << 1); // => 0000_0100 << 1 = 0000_1000 = 8
    println!("{}", x >> 1); // => 0000_0100 >> 1 = 0000_0010 = 2

    println!("-----\n");
}