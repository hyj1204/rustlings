// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    //unwrap 是一种快捷写法， 意思是用match检查有没有这个值，有的话就返回里面的值，没有的话就返回一个panic
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); //to_uppercase 是返回一个新值，而不是在原来的值上面修改

    println!("{}", data);
}
