fn hello(x: &str) {
    println!("Hello, {name}",
        name=x);
}

fn main() {
    let names = vec![
        "Alice",
        "Bob",
        "Charlie"];
    for n in names {
        hello(n);
    }
}
