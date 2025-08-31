pub fn exec() {
    println!("FUNCTIONS AND CODE BLOCKS");
    my_fnc("This is my function");
    let str = "Function call with a variable";
    my_fnc(str);

    let answer =multiplication(10, 15);
    println!("Answer: {answer}");
    let _result = basic_math(10, 15);
    let (_multiplication, _addition, _subtraction) = basic_math(10, 15);

    let full_name = {
        let first_name = "Nouman";
        let last_name = "Azam";
        format!("{first_name} {last_name}")
    };
    println!("Full name: {full_name}");

    println!("-----\n");
}

fn my_fnc(s: &str) {
    println!("{s}")
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication");
    num1 + num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}