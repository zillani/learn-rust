pub fn run() {
    greeting("Hello", "Shaik!");
    println!("Sum is {}", add(2, 2));
    //Closure
    let add_nums = |x: i32, y: i32| x + y;
    println!("Sum is {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
