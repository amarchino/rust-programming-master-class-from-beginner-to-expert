pub fn exec() {
    println!("OPERATOR PRECEDENCE");
    let _result = 5 + 2 * 3; // 5 + (2 * 3)
    /*
    Operator precedence:
    * / %
    + -
    << >>
    &
    ?
    |
    == != < > <= >=
    &&
    ||
    .. ..=
    = += -= *= /= %=
    &= |= ^= <<= >>=
    */

    let _expression = 16 / 4 * 2; // (16 / 4) * 2
    let _expression = (5 + 2) * 3;
    let a = 3;
    let b = 4;
    let c = 5;
    let result = a + b - c == 2 || a == 3; // (((a + b) - c) == 2) || (a == 3)
    // 3 + 4 = 7
    // 7 - 5 = 2
    // 2 == 2 = true
    // 3 == 3 = true
    // true || true = true
    println!("Result is {result}");

    println!("-----\n");
}