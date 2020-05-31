pub fn run() {
    let age = 11;

    if age < 18 {
        println!("You need to be above {}", age);
    }
    let if_age = if age < 18 { true } else { false };
    print!("{}", if_age);
}
