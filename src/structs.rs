pub fn run() {
    let c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("{}, {}, {}", c.red, c.green, c.blue);

    let p = Person::new("zillani", "shaik");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name())

}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//This is like interface
struct Person {
    first_name: String,
    last_name: String,
}

//This is like class
impl Person {
    //This is like constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        //string concatenation
        format!("{}{}", &self.first_name, &self.last_name)
    }
}
