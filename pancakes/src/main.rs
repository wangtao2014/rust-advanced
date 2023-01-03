use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[allow(dead_code)]
#[derive(HelloMacro)]
enum Apples {
    Red,
    Green,
}

fn main() {
    Pancakes::hello_macro();
    Apples::hello_macro();
}


