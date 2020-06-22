use std::env;

fn main() {
    println!("hello from rust");
    for (i, arg) in env::args().enumerate() {
        println!("  {} : {}", i, arg);
    }
}
