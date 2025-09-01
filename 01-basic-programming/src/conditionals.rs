pub fn exec() {
    println!("CONDITIONALS");

    let num = 40;

    if num < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is greater or equal to 50")
    }

    let marks = 95;

    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'F'
    };
    println!("Marks: {marks} - Grade: {grade}");

    let marks = 95;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F'
    };
    println!("Marks: {marks} - Grade: {grade}");

    println!("-----\n");
}