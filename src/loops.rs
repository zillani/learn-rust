pub fn run() {
    let mut count = 0;
    //infinite loop
    loop {
        count += 1;
        print!("{} ", count);
        if count > 10 {
            break;
        };
    }
    println!();
    count = 0;
    while count <= 10 {
        count += 1;
        print!("{} ", count);
    }
    println!();
    for x in 1..10 {
        print!("{} ", x);
    }
}
