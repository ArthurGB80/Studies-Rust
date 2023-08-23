enum Direction {
    left,
    right,
    up,
}

fn main() {
    let go = Direction::left; 
    match go {
        Direction::left => println!("go left"),
        Direction::right => println!("go right"),
        Direction::up => println!("go up"),
    }

}