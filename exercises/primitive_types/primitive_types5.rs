// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
    // 用这个方法提取tuple里面的数据显示出来
    println!("{} is {} years old.", name, age);
}
