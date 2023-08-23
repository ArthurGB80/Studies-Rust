
    /*
    let coord = (2, 3);
    println!("{:?}", "{:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}", "{:?}", x, y);

    let (name, age) = ("Emma", 20);

    let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");

    let state = favorites.2;
    let place = favorites.5;*/

fn coordinate() -> (i32,i32) {
    (1,7)
       
}

fn main () {
    let (x, y) = coordinate();

    if y > 5 {
        println!("y is greater than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is equal to 5");
    }
}
