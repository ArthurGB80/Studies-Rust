enum Colors {
    Red,
    Green,
    Blue,
}

fn print_color(my_color: Colors) {
    match my_color {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
    }

fn main() {
    print_color(Colors::Blue);

}