use backend::hello_from_backend;

fn main() {
    println!("Hello from CLI!");
    println!("{}", hello_from_backend());
}