fn main() {
    let collection = vec!["hello", "Rust", "world"];
    let item = "Rust";

    let found = collection.iter().find(|i| i == item);
    if let Some("Rust") = found {
        println!("We found Rust!");
    }
}
