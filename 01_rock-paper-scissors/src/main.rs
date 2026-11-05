use std::io;

fn main() {
    println!("Choose one: rock / paper / scissors");

    let mut decision=String::new();

    io::stdin().read_line(&mut decision).expect("Input failed");

    println!("Your choice: {decision}");
}
