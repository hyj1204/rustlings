// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn main() {
    //变量必需用mut，因为默认都是不能变的variable
    let mut x = 11;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
