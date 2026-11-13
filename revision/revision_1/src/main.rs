use std::io;
fn main() {
    let mut reed = String::new();
    io::stdin().read_line(& mut reed).expect("i don't know");
    let d :char= reed.trim().parse().expect("wat up");
       println!("Hello, world!");
}
