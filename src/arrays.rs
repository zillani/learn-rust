use std::mem;

pub fn run() {
    let numbers: [i32; 3] = [1, 2, 3];
    println!("{:?}", numbers);

    //Arrays are stack allocated
    println!("Size of array is {} bytes ", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice is {:?}", slice);
}
