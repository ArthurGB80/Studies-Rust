struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let cereals = GroceryItem {
        stock: 10,
        price: 3.99,
    };
    println!("stock: {}", cereals.stock);
    println!("price:{}", cereals.price);
}