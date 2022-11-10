// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    //重新用let 用同一个名字，但是数据改了，这个叫shadow
    //此时是重新建立同一名字的variable,并给一个新值。 原来的那个变量还是原来的,但只存在在当前的scope
    {
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
    println!("{number}");
    /*
    Spell a Number : T-H-R-E-E
    Number plus two is : 5
    T-H-R-E-E
         */
}
