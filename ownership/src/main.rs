use std::io;

fn main() {
    let mut my_string = String::new();

    println!("Enter any string you wish: ");

    io::stdin()
        .read_line(&mut my_string)
        .expect("Failed to read line");

    println!("Your string is {}", &my_string);
    string_is_longer_than_n(&my_string, 2);
    println!("Your string is still {}", &my_string);
}

fn string_is_longer_than_n(string: &str, n: usize) {
    let len = string.len();
    if len > n {
        println!("Yes");
    } else {
        println!("No")
    }
}
