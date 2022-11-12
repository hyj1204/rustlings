// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

fn main() {
    // Characters (`char`)
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '2';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        // char 是有检测里面是单个数字还是单个字母的能力
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
