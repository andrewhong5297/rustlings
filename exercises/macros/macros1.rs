// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// this is a declaritive macro, which works similar to a match expression. here there is just one option () and one output.
// macro matching patterns are defined here https://doc.rust-lang.org/reference/macros-by-example.html
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!(); //was missing !
}
