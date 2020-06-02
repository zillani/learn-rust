enum Movement {
    Up, 
    Down, 
    Left,
    Right
} 

fn move_dir(m: Movement){
    match m{
        Movement::Up => println!("Move Up"),
        Movement::Down => println!("Move Down"),
        Movement::Left => println!("Move Left"),
        Movement::Right => println!("Move Right")
    }

}
pub fn run(){
    move_dir(Movement::Up);
}