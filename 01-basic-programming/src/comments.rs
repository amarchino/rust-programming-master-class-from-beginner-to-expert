pub fn exec() {
    println!("COMMENTS");

    // The current line is a comment line
    // This is the second line of comment
    /* This is a
    multiple line
    comment */

    print!("This is a print command");
    print!("This is going to be printed on the same line");

    println!("\nWill be printed after one empty line");
    println!("\tA tab space at the start");
    println!("This will be overwritten \rOnly this text will appear on the screen");
    println!("Prints double quotes \", Print backslash \\");

    println!(
        "I am doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );
    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "code",
        language = "Rust"
    );

    // let mut n = String::new();
    // std::io::stdin()
    //     .read_line(&mut n)
    //     .expect("Failed to read input.");
    let n = String::from("12");
    let n: f64 = n.trim().parse().expect("Invalid input");
    println!("{n}");

    let _x = 40_000;

    static WELCOME: &str = "Welcome to Rust";
    const PI: f32 = 3.14;

    let _a = PI;
    let _b = PI;
    let _c = WELCOME;
    let _d = WELCOME;

    println!("-----\n");
}