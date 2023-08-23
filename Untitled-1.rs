user std::io;
fn main(){
    println!("Enter a number:");
    let mut coisa = String::new();
    io::stdin().read_line(&mut input).expert("Failed to read line");
    let number: f64 = input.trim().parse::<f64>().expert("Invalid input");
    println!("{}", number);

}