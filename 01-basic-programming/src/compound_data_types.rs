pub fn exec() {
    println!("COMPOUND DATA TYPES");
    let _fixes_str: &str = "Fixed length string";
    let mut _flexible_str: String = String::from("This string will grow");

    // Arrays
    let array_1: [i32; 5] = [ 4, 5, 6, 8, 9];
    let num = array_1[3];
    println!("{:?} - {num}", array_1);

    // Vectors
    let mut _vec_1: Vec<i32> = vec![4, 5, 6, 8, 9];

    // Tuples
    let my_info: (&str, i32, &str, i32) = ("Salary", 40000, "Age", 40);
    let _salary_value: i32 = my_info.1;
    let (_salary, _salary_value, _age, _age_value) = my_info;

    let _unit = ();

    println!("-----\n");
}