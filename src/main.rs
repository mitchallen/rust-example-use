fn main() {
    println!("Coin Flip Demo");

    let mut bool_vector: Vec<bool> = Vec::new();

    for _ in 0..20 {
        bool_vector.push(rust_example_lib::coin::flip());
    }

    println!("{:?}", bool_vector);
}
