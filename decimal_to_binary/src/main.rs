use std::io::stdin;

fn decimal_to_binary(decimal: u32) -> String {
    let mut binary = String::new();
    let mut decimal = decimal;
    while decimal > 0 {
        binary.push_str(&format!("{}", decimal % 2));
        decimal /= 2;
    }
    let binary = binary.chars().rev().collect::<String>();
    binary
}

fn main() {
    let mut decimal = String::new();

    println!("Enter a decimal number: ");
    stdin()
        .read_line(&mut decimal)
        .ok()
        .expect("Failed to read line");
    let binary = decimal_to_binary(decimal.trim().parse().unwrap());
    println!("{binary}")
}
