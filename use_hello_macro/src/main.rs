use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakez;

fn main() {
    Pancakez::hello_macro();
}
