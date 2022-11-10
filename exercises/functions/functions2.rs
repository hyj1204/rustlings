// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(4);
}

fn call_me(num: u32) {
    println!("{:?}", 0..num); // 0..4 里面是0，1，2 3，4  不包括4
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
