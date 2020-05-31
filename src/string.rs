pub fn run(){
    let mut hello = String::from("Hello");
    hello.push_str(", World");
    println!("Message is {}", hello);

    for word in hello.split_whitespace(){
        println!("{}", word);
    }
}