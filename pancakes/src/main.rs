use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[allow(dead_code)]
#[derive(HelloMacro)]
enum Apples {}

fn main() {
    Pancakes::hello_macro();
    Apples::hello_macro();
}


