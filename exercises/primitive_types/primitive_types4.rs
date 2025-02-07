// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    // nice_slice 存的是个指针，里面指向index 1-3,是不包括index 4的数据（也就是最后一个5）

    assert_eq!([2, 3, 4], nice_slice)
}
