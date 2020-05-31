use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", numbers);

    //Vectors are stack allocated
    println!("Size of vector is {} bytes ", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice is {:?}", slice);

    //Add to vector
    numbers.push(4);
    numbers.push(5);
    for x in numbers.iter() {
        print!("{} ", x);
    }

    print!("{:?} ", numbers);
}
