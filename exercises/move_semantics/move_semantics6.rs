// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    println!("The last char of the data is : {}.", get_char(&data.clone()));

    println!("The value of data is : {}", data);

    string_uppercase(data);

    // println!("{}", data); // string_uppercase takes ownership of the data.
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data.to_uppercase();

    println!("Uppercase of the data : {}", data);
}
